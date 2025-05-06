use legion::{system, systems::CommandBuffer, world::SubWorld, Entity};
use macroquad::math::{IVec2, Rect, Vec2};
use legion::query::*;
use crate::{comps::*, resources::{chunk_manager::ChunkManager, ivec2_to_vec2, vec2_to_ivec2}};



fn get_chunk_by_position(pos: Vec2, cm: &ChunkManager) -> IVec2 {
    let chunk_m = cm.chunk_size_in_meters;
    vec2_to_ivec2((pos / chunk_m).floor())
}

fn get_chunk_rect(pos: IVec2, cm: &ChunkManager) -> Rect {
    let pos = ivec2_to_vec2(pos) * cm.chunk_size_in_meters;
    let size = cm.chunk_size_in_meters;

    Rect::new(pos.x, pos.y, size.x, size.y)
}


#[system(for_each)]
pub fn update_player_chunk(
    t: &Transform,
    _: &Player,
    #[resource] cm: &mut ChunkManager,
) {
    cm.player_chunk = get_chunk_by_position(t.position, cm);
}

#[system(for_each)]
pub fn update_monster_chunk(
    t: &Transform,
    m: &mut Monster,
    #[resource] cm: &ChunkManager,
) {
    m.chunk = get_chunk_by_position(t.position, cm)
}

#[system]
#[read_component(Chunk)]
#[read_component(Monster)]
pub fn load_freed_chunks(
    world: &mut SubWorld,
    #[resource] cm: &ChunkManager,
    cb: &mut CommandBuffer,
) {
    let load_radius = (cm.unloading_distance as f32).sqrt().ceil() as i32;
    for dy in -load_radius..=load_radius {
        for dx in -load_radius..=load_radius {
            let pos = cm.player_chunk + IVec2::new(dx, dy);

            if pos.distance_squared(cm.player_chunk) < cm.unloading_distance
                && !<&Chunk>::query().iter(world).find(|c| c.pos == pos).is_some()
            {
                let rect = get_chunk_rect(pos, cm);
                let mut chunk = Chunk::new(pos, rect);
                chunk.load(&cm.world_noise, &world, cm, cb);
                cb.push((
                    chunk,
                ));
            }
        }
    }
}

#[system]
#[write_component(Chunk)]
#[read_component(Monster)]
pub fn load_chunks(
    world: &mut SubWorld,
    #[resource] cm: &ChunkManager,
    cb: &mut CommandBuffer,
) {
    let chunks_to_load: Vec<_> = <&mut Chunk>::query()
        .iter_mut(world)
        .filter(|chunk| chunk.pos.distance_squared(cm.player_chunk) < cm.unloading_distance)
        .filter(|chunk| chunk.state != ChunkState::Loaded)
        .map(|chunk| chunk as *mut Chunk) 
        .collect();

    for chunk_ptr in chunks_to_load {
        let chunk = unsafe { &mut *chunk_ptr };
        chunk.load(&cm.world_noise, world, cm, cb);
    }
}

#[system]
#[write_component(Chunk)]
#[read_component(Monster)]
pub fn unload_chunks(
    world: &mut SubWorld,
    #[resource] cm: &ChunkManager,
    cb: &mut CommandBuffer,
) {
    let chunks_to_unload: Vec<_> = <&mut Chunk>::query()
        .iter_mut(world)
        .filter(|chunk| chunk.pos.distance_squared(cm.player_chunk) >= cm.unloading_distance)
        .filter(|chunk| chunk.state == ChunkState::Loaded)
        .map(|chunk| chunk as *mut Chunk) 
        .collect();

    for chunk_ptr in chunks_to_unload {
        let chunk = unsafe { &mut *chunk_ptr };
        chunk.unload(world, cb);
    }
}

#[system]
#[write_component(Chunk)]
#[read_component(Transform)]
pub fn free_chunks(
    world: &mut SubWorld,
    #[resource] cm: &ChunkManager,
    commands: &mut CommandBuffer,
) {
    // 1) Coleto todos os chunks que devem ser “free” e suas entidades
    let to_free: Vec<(Entity, IVec2)> = <(Entity, &Chunk)>::query()
        .iter(world)
        .filter_map(|(e, chunk)| {
            if chunk.pos.distance_squared(cm.player_chunk) >= cm.freeing_distance {
                Some((*e, chunk.pos))
            } else {
                None
            }
        })
        .collect();

    // 2) Para cada chunk:
    for (chunk_entity, chunk_pos) in to_free {
        // - 2a) retire todas as entidades “filhas” dentro do retângulo do chunk
        let rect = get_chunk_rect(chunk_pos, cm);
        <(Entity, &Transform)>::query()
            .iter(world)
            .filter(|(_, t)| rect.contains(t.position))
            .for_each(|(entity, _)| {
                commands.remove(*entity);
            });

        // - 2b) finalmente, remova o próprio chunk do World
        commands.remove(chunk_entity);
    }
}