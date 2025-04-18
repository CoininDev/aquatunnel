use std::collections::HashMap;

use glam::{IVec4, Vec2, vec2};
use legion::storage::Component;
use sdl2::{pixels::Color, rect::FPoint};

#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Sprite {
    pub image_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DebugSprite {
    pub size: FPoint,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spritesheet {
    pub animations: HashMap<String, Vec<IVec4>>,
    pub image_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationPlayer {
    pub current_animation: String,
    pub current_frame: usize,
    pub playing: bool,
    pub frame_duration: f32,
}
