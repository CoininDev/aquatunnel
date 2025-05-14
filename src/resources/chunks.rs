use std::collections::HashMap;

use legion::{ query::*, world::SubWorld};
use macroquad::math::{IVec2, Rect, Vec2};

use crate::comps::{Player, Switch, Transform};

fn ivec2_to_vec2(from: IVec2) -> Vec2 {
    Vec2::new(from.x as f32, from.y as f32)
}

fn vec2_to_ivec2(from: Vec2) -> IVec2 {
    IVec2::new(from.x as i32, from.y as i32)
}

pub struct ChunkManager {
    pub chunks: HashMap<IVec2, Chunk>,
    /// This noise will be the base for the world, high values == block, low values == nothing/water.
    pub format_noise: noise::Simplex,
    pub threshold: f32,
    pub player_chunk: IVec2,
    pub chunk_size_in_tiles: IVec2,
    pub tile_size_in_meters: Vec2,
    pub unloading_distance: i32,
    pub freeing_distance: i32,
}

impl ChunkManager {
    pub fn new(
        format_noise: noise::Simplex,
        threshold: f32,
        chunk_size_in_tiles: IVec2,
        tile_size_in_meters: Vec2,
        unloading_distance: i32,
        freeing_distance: i32,
    ) -> Self {
        ChunkManager {
            chunks: HashMap::new(),
            format_noise,
            threshold,
            player_chunk: IVec2::ZERO,
            chunk_size_in_tiles,
            tile_size_in_meters,
            unloading_distance,
            freeing_distance,
        }
    }

    pub fn run(&mut self, world: &mut SubWorld) {
        self.update_player_chunk(world);
        //load new freed chunks
        let load_radius = (self.unloading_distance as f32).sqrt().ceil() as i32;
        for dy in -load_radius..=load_radius {
            for dx in -load_radius..=load_radius {
                let pos = self.player_chunk + IVec2::new(dx, dy);

                if pos.distance_squared(self.player_chunk) < self.unloading_distance
                    && !self.chunks.contains_key(&pos)
                {
                    let rect = self.get_chunk_rect(pos);
                    let chunk = Chunk::new(pos, rect);
                    self.chunks.insert(pos, chunk);
                }
            }
        }

        //load next unloaded chunks
        self.chunks
            .iter_mut()
            .filter(|(pos, _)| pos.distance_squared(self.player_chunk) < self.unloading_distance)
            .filter(|(_, chunk)| chunk.state != ChunkState::Loaded)
            .for_each(|(_, chunk)| chunk.load(&self.format_noise, world));
        //unload distant chunks
        self.chunks
            .iter_mut()
            .filter(|(pos, _)| pos.distance_squared(self.player_chunk) >= self.unloading_distance)
            .filter(|(_, chunk)| chunk.state == ChunkState::Loaded)
            .for_each(|(_, chunk)| chunk.unload(world));
        //free distant chunks
        self.chunks.retain(|&pos, chunk| {
            let dist = pos.distance_squared(self.player_chunk);
            if dist >= self.freeing_distance {
                chunk.free(world);
                return false;
            }
            true
        });
    }

    fn update_player_chunk(&mut self, world: &SubWorld) {
        let mut query = <(&Transform, &Player)>::query();
        let (t, _) = query.iter(world).last().expect("There is no player!");
        self.player_chunk = self.get_chunk_by_position(t.position);
    }

    fn get_chunk_by_position(&self, pos: Vec2) -> IVec2 {
        let chunk_m = ivec2_to_vec2(self.chunk_size_in_tiles) * self.tile_size_in_meters;
        vec2_to_ivec2((pos / chunk_m).floor())
    }

    fn get_chunk_rect(&self, pos: IVec2) -> Rect {
        let pos =
            ivec2_to_vec2(pos) * ivec2_to_vec2(self.chunk_size_in_tiles) * self.tile_size_in_meters;
        let size = ivec2_to_vec2(self.chunk_size_in_tiles) * self.tile_size_in_meters;

        Rect::new(pos.x, pos.y, size.x, size.y)
    }
}

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

    pub fn load(&mut self, noise: &noise::Simplex, world: &mut SubWorld) {
        match self.state {
            ChunkState::Freed => self.spawn(),
            ChunkState::Unloaded => self.for_each_switch_entity_in_chunk(world, |s| s.on = true),
            ChunkState::Loaded => return,
        }

        //TODO: load matrix
        self.state = ChunkState::Loaded;
    }

    pub fn unload(&mut self, world: &mut SubWorld) {
        if self.state == ChunkState::Unloaded {
            return;
        }

        self.matrix = None;
        self.for_each_switch_entity_in_chunk(world, |switch| switch.on = false);
        self.state = ChunkState::Unloaded;
    }

    pub fn free(&self, world: &mut SubWorld) {
        self.for_each_switch_entity_in_chunk(world, |switch| switch.destroy());
    }

    fn for_each_switch_entity_in_chunk<F>(&self, world: &mut SubWorld, mut f: F)
    where
        F: FnMut(&mut Switch),
    {
        let mut query = <(&Transform, &mut Switch)>::query();
        query
            .iter_mut(world)
            .filter(|(t, _)| self.rect.contains(t.position))
            .map(|(_, s)| s)
            .for_each(|s| f(s));
    }

    fn spawn(&self) {
        todo!();
    }
}

/// a Chunk can be in 3 states:
/// Loaded: It means that each tile are in memory and entities are running
/// Unloaded: It still is in memory, but without the tiles (matrix = None), and entities are turned
/// off.
/// Freed: It means that the Chunk is not in the memory (in the chunks variable) anymore.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Loaded,
    Unloaded,
    Freed,
}

