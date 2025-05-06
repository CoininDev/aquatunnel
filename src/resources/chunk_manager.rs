use std::collections::HashMap;

use macroquad::math::{IVec2, Vec2};

use crate::comps::Chunk;

use super::ivec2_to_vec2;

pub struct ChunkManager {
    pub world_noise: noise::Simplex,
    pub threshold: f64,
    pub player_chunk: IVec2,
    pub chunk_size_in_tiles: IVec2,
    tile_size_in_meters: Vec2,
    pub chunk_size_in_meters: Vec2,
    pub unloading_distance: i32,
    pub freeing_distance: i32,
}

impl ChunkManager {
    pub fn new(
        world_noise: noise::Simplex,
        threshold: f64,
        chunk_size_in_tiles: IVec2,
        tile_size_in_meters: Vec2,
        unloading_distance: i32,
        freeing_distance: i32,
    ) -> Self{
        Self {
            world_noise, 
            threshold, 
            player_chunk: IVec2::ZERO, 
            chunk_size_in_tiles, 
            tile_size_in_meters,
            chunk_size_in_meters: ivec2_to_vec2(chunk_size_in_tiles) * tile_size_in_meters, 
            unloading_distance, 
            freeing_distance 
        }
    }
}