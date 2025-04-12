use glam::{Vec2, vec2};
use sdl2::{pixels::Color, rect::FPoint};

pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
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

pub struct Sprite {
    pub image_path: String
}

pub struct DebugSprite {
    pub size: FPoint,
    pub color: Color,
}
