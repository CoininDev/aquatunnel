use legion::{Entity, query::*, systems::CommandBuffer, world::SubWorld};
use macroquad::math::{IVec2, Rect, UVec2, Vec2, uvec2, vec2};
use nalgebra::vector;
use rapier2d::prelude::{
    Collider, ColliderBuilder, ColliderHandle, RigidBody, RigidBodyBuilder, RigidBodyHandle,
};

use crate::{
    common::Matrix,
    resources::{chunk_manager::ChunkManager, physics::PhysicsContext},
};

use super::Monster;

pub fn calculate_tile_position(
    chunk_pos: IVec2,
    tile_pos: UVec2,
    chunk_size_tiles: UVec2,
    tile_size_meters: Vec2,
) -> Vec2 {
    vec2(
        (chunk_pos.x * chunk_size_tiles.x as i32 + tile_pos.x as i32) as f32 * tile_size_meters.x,
        (chunk_pos.y * chunk_size_tiles.y as i32 + tile_pos.y as i32) as f32 * tile_size_meters.y,
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub pos: IVec2,
    pub matrix: Option<Matrix<u32>>,
    pub state: ChunkState,
    /// in meters
    pub rect: Rect,
}

impl Chunk {
    pub fn new(pos: IVec2, rect: Rect) -> Self {
        Chunk {
            pos,
            matrix: None,
            state: ChunkState::Freed,
            rect,
        }
    }

    pub fn load(&self, e: &Entity, world: &SubWorld, cm: &ChunkManager, cb: &mut CommandBuffer) {
        if self.state == ChunkState::Loaded {
            return;
        }

        let matrix = self.gen_matrix(cm);

        if self.state == ChunkState::Unloaded {
            self.set_inchunk_monsters_active(world, cb, true);
        }
        if self.state == ChunkState::Freed {
            self.spawn(cb);
        }

        cb.add_component(
            *e,
            Chunk {
                state: ChunkState::Loaded,
                matrix: Some(matrix),
                ..self.clone()
            },
        );
    }

    pub fn unload(&self, e: &Entity, world: &SubWorld, cb: &mut CommandBuffer) {
        if self.state != ChunkState::Loaded {
            return;
        }
        self.set_inchunk_monsters_active(world, cb, false);
        cb.add_component(
            *e,
            Chunk {
                state: ChunkState::Unloaded,
                matrix: None,
                ..self.clone()
            },
        );
    }

    pub fn free(&self, e: &Entity, world: &SubWorld, cb: &mut CommandBuffer) {
        self.destroy_inchunk_monsters(world, cb);
        cb.remove(*e);
    }

    //=====PRIVATE======
    fn set_inchunk_monsters_active(&self, world: &SubWorld, cb: &mut CommandBuffer, active: bool) {
        let mut q = <(Entity, &Monster)>::query();

        for (entity, monster) in q.iter(world) {
            if monster.chunk == self.pos {
                let mut new_monster = monster.clone();
                new_monster.active = active;
                cb.add_component(*entity, new_monster);
            }
        }
    }

    fn gen_matrix(&self, cm: &ChunkManager) -> Matrix<u32> {
        let mut matrix_buffer: Matrix<u32> = Matrix::new(
            (cm.chunk_size_in_tiles.x + 1) as usize,
            (cm.chunk_size_in_tiles.y + 1) as usize,
            0,
        );

        for y in 0..=cm.chunk_size_in_tiles.y as usize {
            for x in 0..=cm.chunk_size_in_tiles.x as usize {
                let world_pos = calculate_tile_position(
                    self.pos,
                    uvec2(x as u32, y as u32),
                    cm.chunk_size_in_tiles,
                    cm.tile_size_in_meters,
                );
                let world_x: f32 = world_pos.x * cm.noise_scale.x;
                let world_y: f32 = world_pos.y * cm.noise_scale.y;
                let noise_val = cm.world_noise.get_noise_2d(world_x, world_y);
                matrix_buffer[(x, y)] = if noise_val < cm.threshold { 0 } else { 1 };
            }
        }
        matrix_buffer
    }

    fn destroy_inchunk_monsters(&self, world: &SubWorld, cb: &mut CommandBuffer) {
        let mut q = <(Entity, &Monster)>::query();
        for (e, m) in q.iter(world) {
            if m.chunk == self.pos {
                cb.remove(*e);
            }
        }
    }

    // Será implementada quando os monstros estiverem prontos
    fn spawn(&self, _cb: &mut CommandBuffer) {}
}

/// a Chunk can be in 3 states:
/// Loaded: It means that each tile are in memory and entities are running
/// Unloaded: It still is in memory, but without the tiles (matrix = None), and entities are turned
/// off.
/// Freed: It means that the Chunk is not in the memory (in the chunks variable) anymore.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChunkState {
    Loaded,
    Unloaded,
    Freed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkBody {
    pub pos: IVec2,
    pub state: ChunkState,
    pub body_handles: Matrix<Option<RigidBodyHandle>>,
    pub collider_handles: Matrix<Option<ColliderHandle>>,
}

impl ChunkBody {
    pub fn new(size: UVec2, pos: IVec2) -> Self {
        Self {
            body_handles: Matrix::new(size.x as usize, size.y as usize, None),
            collider_handles: Matrix::new(size.x as usize, size.y as usize, None),
            pos,
            state: ChunkState::Freed,
        }
    }

    pub fn load(
        &self,
        e: &Entity,
        chunk: &Chunk,
        cm: &ChunkManager,
        pc: &mut PhysicsContext,
        cb: &mut CommandBuffer,
    ) {
        if self.state == ChunkState::Loaded {
            return;
        }

        if let Ok((body_handles, collider_handles)) = self.gen_matrix(chunk, cm, pc) {
            cb.add_component(
                *e,
                ChunkBody {
                    body_handles,
                    collider_handles,
                    state: ChunkState::Loaded,
                    ..self.clone()
                },
            );
        }
    }

    pub fn unload(
        &self,
        e: &Entity,
        cm: &ChunkManager,
        pc: &mut PhysicsContext,
        cb: &mut CommandBuffer,
    ) {
        if self.state != ChunkState::Loaded {
            return;
        }

        self.clear_matrix(cm, pc);

        cb.add_component(
            *e,
            ChunkBody {
                body_handles: Matrix::new(
                    cm.chunk_size_in_tiles.x as usize,
                    cm.chunk_size_in_tiles.y as usize,
                    None,
                ),
                collider_handles: Matrix::new(
                    cm.chunk_size_in_tiles.x as usize,
                    cm.chunk_size_in_tiles.y as usize,
                    None,
                ),
                state: ChunkState::Unloaded,
                ..self.clone()
            },
        );
    }

    fn gen_matrix(
        &self,
        chunk: &Chunk,
        cm: &ChunkManager,
        pc: &mut PhysicsContext,
    ) -> Result<(Matrix<Option<RigidBodyHandle>>, Matrix<Option<ColliderHandle>>), String> {
        let size_x = (cm.chunk_size_in_tiles.x + 1) as usize;
        let size_y = (cm.chunk_size_in_tiles.y + 1) as usize;

        let mut rb_matrix   = Matrix::new(size_x, size_y, None);
        let mut col_matrix  = Matrix::new(size_x, size_y, None);

        let og_matrix = match &chunk.matrix {
            Some(m) => m,
            _ => return Err("Chunk sem matrix ainda".into()),
        };

        for y in 0..og_matrix.height {
            for x in 0..og_matrix.width {
                let tile = og_matrix[(x, y)];
                let tile_pos = UVec2::new(x as u32,y as u32);
                if tile == 0 {
                    continue;
                }
                
                let rb = self.create_new_tile_body(tile_pos, cm);
                let col = self.create_new_tile_collider(cm);
                let (rb_handle, col_handle) = self.insert_tile(rb, col, pc);
                rb_matrix[(x,y)] = Some(rb_handle);
                col_matrix[(x,y)] = Some(col_handle);
            }
        }

        Ok((rb_matrix, col_matrix))
    }



    fn clear_matrix(&self, cm: &ChunkManager, pc: &mut PhysicsContext) {
        for y in 0..=cm.chunk_size_in_tiles.y as usize {
            for x in 0..=cm.chunk_size_in_tiles.x as usize {
                let mut rigid_bodies = pc.bodies.borrow_mut();
                if let Some(pinto_grosso) = self.body_handles.get(x, y) {
                    if let Some(rb_handle) = pinto_grosso {
                        rigid_bodies.remove(
                            *rb_handle,
                            &mut pc.islands.borrow_mut(),
                            &mut pc.colliders.borrow_mut(),
                            &mut pc.impulse_joints.borrow_mut(),
                            &mut pc.multibody_joints.borrow_mut(),
                            true,
                        );
                    }
                }
            }
        }
    }

    fn insert_tile(
        &self,
        rb: RigidBody,
        col: Collider,
        pc: &mut PhysicsContext,
    ) -> (RigidBodyHandle, ColliderHandle) {
        let mut rigid_bodies = pc.bodies.borrow_mut();
        let mut colliders = pc.colliders.borrow_mut();
        let rb_handle = rigid_bodies.insert(rb);
        let col_handle = colliders.insert_with_parent(col, rb_handle, &mut rigid_bodies);

        (rb_handle, col_handle)
    }

    
    fn create_new_tile_body(&self, tile_pos: UVec2, cm: &ChunkManager) -> RigidBody {
        let world_origin = calculate_tile_position(
            self.pos,
            tile_pos,
            cm.chunk_size_in_tiles,
            cm.tile_size_in_meters,
        );
        // desloca para o centro do tile:
        let half = cm.tile_size_in_meters / 2.0;
        let center = vec2(world_origin.x + half.x, world_origin.y + half.y);

        RigidBodyBuilder::fixed()
            .translation(vector![center.x, center.y])
            .build()
    }


    fn create_new_tile_collider(&self, cm: &ChunkManager) -> Collider {
        ColliderBuilder::cuboid(cm.tile_size_in_meters.x / 2., cm.tile_size_in_meters.y / 2.).build()
    }
}
