pub mod chunks;
pub mod input;
pub mod physics;
pub mod renderable;
use std::collections::HashMap;
use std::sync::Arc;

use crate::comps::Transform;
use macroquad::math::Vec2;
use macroquad::texture::Texture2D;
pub use renderable::METERS_TO_PIXELS;
pub use renderable::Renderable;

pub struct RenderQueue(pub Vec<(&'static Transform, &'static dyn Renderable)>);

pub struct Textures(pub HashMap<String, Arc<Texture2D>>);

pub struct Time {
    pub delta: f32,
}

pub struct Track {
    pub pos: Vec2,
}
