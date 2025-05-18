//pub mod chunks;
pub mod chunk_manager;
pub mod input;
pub mod physics;
pub mod renderable;
use std::collections::HashMap;
use std::sync::Arc;

use macroquad::math::IVec2;
use macroquad::math::UVec2;
use macroquad::math::Vec2;
use macroquad::texture::Texture2D;
pub use renderable::METERS_TO_PIXELS;

pub struct Textures(pub HashMap<String, Arc<Texture2D>>);

pub fn ivec2_to_vec2(from: IVec2) -> Vec2 {
    Vec2::new(from.x as f32, from.y as f32)
}

pub fn uvec2_to_vec2(from: UVec2) -> Vec2 {
    Vec2::new(from.x as f32, from.y as f32)
}

pub fn vec2_to_ivec2(from: Vec2) -> IVec2 {
    IVec2::new(from.x as i32, from.y as i32)
}

pub struct Time {
    pub delta: f32,
}

pub struct Track {
    pub pos: Vec2,
}
