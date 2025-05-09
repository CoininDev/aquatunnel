use std::collections::HashMap;

use super::{ivec2_to_vec2, uvec2_to_vec2};
use fastnoise_lite::FastNoiseLite;
use legion::Entity;
use macroquad::math::{I64Vec2, IVec2, UVec2, Vec2};

pub struct ChunkManager {
    pub chunks: HashMap<IVec2, Entity>,
    pub world_noise: FastNoiseLite,
    pub noise_scale: Vec2,
    pub threshold: f32,
    pub player_chunk: IVec2,
    pub chunk_size_in_tiles: UVec2,
    pub tile_size_in_meters: Vec2,
    pub chunk_size_in_meters: Vec2,
    pub unloading_distance: i32,
    pub freeing_distance: i32,
}

impl ChunkManager {
    pub fn new(
        world_noise: FastNoiseLite,
        noise_scale: Vec2,
        threshold: f32,
        chunk_size_in_tiles: UVec2,
        tile_size_in_meters: Vec2,
        unloading_distance: i32,
        freeing_distance: i32,
    ) -> Self {
        Self {
            chunks: HashMap::new(),
            world_noise,
            noise_scale,
            threshold,
            player_chunk: IVec2::ZERO,
            chunk_size_in_tiles,
            tile_size_in_meters,
            chunk_size_in_meters: uvec2_to_vec2(chunk_size_in_tiles) * tile_size_in_meters,
            unloading_distance,
            freeing_distance,
        }
    }
}
