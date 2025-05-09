use std::collections::HashMap;

use macroquad::{
    color::Color,
    math::{IVec2, IVec4, Vec2},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
    pub image_path: String,
    pub z_order: f32,
    pub flip_x: bool,
    pub flip_y: bool,
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
/// TileMap can be used with local TileMapSource, or alternately based on external info, as chunkmanager
#[derive(Debug, Clone, PartialEq)]
pub struct TileMap {
    pub tileset_path: String,
    pub tiles: HashMap<u32, IVec2>,
    pub tile_size: Vec2,
    pub tile_size_in_tileset: Vec2,
    pub z_order: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileMapSource {
    pub matrix: Vec<Vec<u32>>,
}
