use std::collections::HashMap;

use crate::{
    comps::*,
    resources::{
        chunk_manager::ChunkManager, ivec2_to_vec2, physics::PhysicsContext, vec2_to_ivec2,
    },
};
use legion::query::*;
use legion::{Entity, system, systems::CommandBuffer, world::SubWorld};
use macroquad::math::{IVec2, Rect, Vec2, ivec2};

fn get_chunk_by_position(pos: Vec2, cm: &ChunkManager) -> IVec2 {
    let chunk_m = cm.chunk_size_in_meters;
    vec2_to_ivec2((pos / chunk_m).floor())
}

fn get_chunk_rect(pos: IVec2, cm: &ChunkManager) -> Rect {
    let pos = ivec2_to_vec2(pos) * cm.chunk_size_in_meters;
    let size = cm.chunk_size_in_meters;

    Rect::new(pos.x, pos.y, size.x, size.y)
}

fn get_world_position_by_chunk(chunk: IVec2, cm: &ChunkManager) -> Vec2 {
    ivec2_to_vec2(chunk) * cm.chunk_size_in_meters
}

#[system(for_each)]
pub fn update_player_chunk(t: &Transform, _: &Player, #[resource] cm: &mut ChunkManager) {
    cm.player_chunk = get_chunk_by_position(t.position, cm);
}

#[system(for_each)]
pub fn debug_chunk(c: &Chunk) {
    println!("{:?}", c);
}

#[system(for_each)]
pub fn update_monster_chunk(t: &Transform, m: &mut Monster, #[resource] cm: &ChunkManager) {
    m.chunk = get_chunk_by_position(t.position, cm)
}

fn get_chunk_tiles() -> HashMap<u32, IVec2> {
    let mut cu = HashMap::new();
    cu.insert(0, ivec2(0, 0));
    cu.insert(1, ivec2(5, 5));
    cu
}

#[system]
#[read_component(Chunk)]
#[read_component(Monster)]
pub fn create_new_chunks(
    world: &SubWorld,
    #[resource] cm: &mut ChunkManager,
    cb: &mut CommandBuffer,
) {
    let load_radius = (cm.unloading_distance as f32).sqrt().ceil() as i32;
    for dy in -load_radius..=load_radius {
        for dx in -load_radius..=load_radius {
            let pos = cm.player_chunk + IVec2::new(dx, dy);

            if pos.distance_squared(cm.player_chunk) < cm.unloading_distance
                && !<&Chunk>::query()
                    .iter(world)
                    .find(|c| c.pos == pos)
                    .is_some()
            {
                let rect = get_chunk_rect(pos, cm);
                cm.chunks.insert(
                    pos,
                    cb.push((
                        Chunk::new(pos, rect),
                        ChunkBody::new(cm.chunk_size_in_tiles, pos),
                        Transform {
                            position: get_world_position_by_chunk(pos, cm),
                            ..Default::default()
                        },
                        TileMap {
                            tileset_path: "assets/dungeon_tiles.png".to_string(),
                            tile_size: cm.tile_size_in_meters,
                            tile_size_in_tileset: Vec2::new(8., 8.),
                            tiles: get_chunk_tiles(),
                            z_order: 0.,
                        },
                    )),
                );
            }
        }
    }
}

#[system]
#[read_component(Chunk)]
#[read_component(Monster)]
#[read_component(ChunkBody)]
pub fn load_chunks(
    world: &SubWorld,
    #[resource] cm: &ChunkManager,
    #[resource] pc: &mut PhysicsContext,
    cb: &mut CommandBuffer,
) {
    let chunks_to_load: Vec<_> = <(Entity, &Chunk, &ChunkBody)>::query()
        .iter(world)
        .filter(|(_, chunk, _)| chunk.pos.distance_squared(cm.player_chunk) < cm.unloading_distance)
<<<<<<< HEAD
        //.filter(|(_, chunk, _)| chunk.state != ChunkState::Loaded)
        .collect();

    for (entity, chunk, body) in chunks_to_load {
        body.load(entity, chunk, cm, pc, cb);
        chunk.load(entity, world, cm, cb);
=======
        .filter(|(_, chunk, _)| chunk.state != ChunkState::Loaded)
        .collect();

    for (entity, chunk, body) in chunks_to_load {
        chunk.load(entity, world, cm, cb);
        body.load(entity, cm, pc, cb);
>>>>>>> cc62d05 (Refatora código para otimizar a manipulação de chunks e a renderização de corpos, além de ajustar a escala de ruído e corrigir a inicialização de entidades.)
    }
}

#[system]
#[read_component(Chunk)]
#[read_component(Monster)]
#[read_component(ChunkBody)]
pub fn unload_chunks(
    world: &SubWorld,
    #[resource] cm: &ChunkManager,
    #[resource] pc: &mut PhysicsContext,
    cb: &mut CommandBuffer,
) {
    let chunks_to_unload: Vec<_> = <(Entity, &Chunk, &ChunkBody)>::query()
        .iter(world)
        .filter(|(_, chunk, _)| {
            chunk.pos.distance_squared(cm.player_chunk) >= cm.unloading_distance
        })
        .filter(|(_, chunk, _)| chunk.state == ChunkState::Loaded)
        .collect();

    for (entity, chunk, body) in chunks_to_unload {
        chunk.unload(entity, world, cb);
        body.unload(entity, cm, pc, cb);
    }
}

#[system]
#[read_component(Chunk)]
#[read_component(Transform)]
#[read_component(Monster)]
pub fn free_chunks(world: &SubWorld, #[resource] cm: &ChunkManager, cb: &mut CommandBuffer) {
    let chunks_to_free: Vec<_> = <(Entity, &Chunk)>::query()
        .iter(world)
        .filter(|(_, chunk)| chunk.pos.distance_squared(cm.player_chunk) >= cm.freeing_distance)
        .collect();

    for (entity, chunk) in chunks_to_free {
        chunk.free(entity, world, cb);
    }
}
