use legion::{query::*, systems::CommandBuffer, world::SubWorld, Entity};
use macroquad::{
    color::Color,
    math::{vec2, IVec2, IVec4, Rect, Vec2},
};
use noise::NoiseFn;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use std::collections::HashMap;

use crate::resources::chunk_manager::ChunkManager;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub speed: f32,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec2(0.0, 0.0),
            scale: vec2(1.0, 1.0),
            rotation: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    pub image_path: String,
    pub z_order: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugSprite {
    pub size: Vec2,
    pub color: Color,
    pub z_order: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spritesheet {
    pub animations: HashMap<String, Vec<IVec4>>,
    pub image_path: String,
    pub dst_size: Vec2,
    pub z_order: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationPlayer {
    pub current_animation: String,
    pub current_frame: usize,
    pub playing: bool,
    pub frame_duration: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub body_handle: Option<RigidBodyHandle>,
    pub collider_handle: Option<ColliderHandle>,
    pub is_dynamic: bool,
    pub size: Vec2,
}

impl Body {
    pub fn new(size: Vec2, is_dynamic: bool) -> Self {
        Body {
            body_handle: None,
            collider_handle: None,
            size,
            is_dynamic,
        }
    }
}

/// TileMap can be used with local TileMapSource, or alternately based on external info, as chunkmanager
#[derive(Debug, Clone, PartialEq)]
pub struct TileMap {
    pub tileset_path: String,
    pub tiles: HashMap<u32, IVec2>,
    pub tile_size: Vec2,
    pub z_order: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileMapSource {
    pub matrix: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub active: bool,
    pub chunk: IVec2
}

impl Monster {
    pub fn destroy(&self) {
        todo!();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub pos: IVec2,
    pub matrix: Option<Vec<Vec<u32>>>,
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

    pub fn load(&mut self, noise: &noise::Simplex, world: &SubWorld, cm: &ChunkManager, cb: &mut CommandBuffer) {
        match self.state {
            ChunkState::Freed => {
                self.spawn(cb);
                self.load_matrix(noise, cm);
                self.state = ChunkState::Loaded;
            },
            ChunkState::Unloaded => {
                self.set_inchunk_monsters_active(world, cb, true);
                self.load_matrix(noise, cm);                
                self.state = ChunkState::Loaded;
            },
            ChunkState::Loaded => return,
        }
    }

    pub fn unload(&mut self, world: &SubWorld, cb: &mut CommandBuffer) {
        if self.state == ChunkState::Unloaded {
            return;
        }

        self.matrix = None;
        self.set_inchunk_monsters_active(world, cb, false);
        self.state = ChunkState::Unloaded;
    }

    pub fn free(&self, world: &SubWorld, cb: &mut CommandBuffer) {
        self.destroy_inchunk_monsters(world, cb);
    }
    
    
    //=====PRIVATE======
    fn set_inchunk_monsters_active(&self, world: &SubWorld, cb: &mut CommandBuffer, active: bool) {
        let mut q = <(Entity, &Monster)>::query();

        for (entity, monster) in q.iter(world) {
            if monster.chunk == self.pos {
                let mut new_monster = monster.clone(); // requer Clone em Monster
                new_monster.active = active;
                cb.add_component(*entity, new_monster);
            }
        }
    }
    
    
    fn load_matrix(&mut self, noise: &noise::Simplex, cm: &ChunkManager) {
        let mut matrix_buffer: Vec<Vec<u32>> = vec![vec![0; cm.chunk_size_in_tiles.x as usize + 1]; cm.chunk_size_in_tiles.y as usize + 1];
        for y in 0..=cm.chunk_size_in_tiles.x as usize {
            for x in 0..=cm.chunk_size_in_tiles.y as usize {
                let noise_val = noise.get([x as f64, y as f64]);
                if noise_val < cm.threshold {
                    matrix_buffer[y][x] = 0;
                } else {
                    matrix_buffer[y][x] = 1;
                }
            }
        }

        self.matrix = Some(matrix_buffer.clone());
    }

    fn destroy_inchunk_monsters(&self, world: &SubWorld, cb: &mut CommandBuffer) {
        let mut q = <(Entity, &Monster)>::query();
        for (e, m) in q.iter(world) {
            if m.chunk == self.pos {
                cb.remove(*e);
            }
        }
    }

    fn spawn(&self, cb: &mut CommandBuffer) {
        return
    }
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