use std::collections::HashMap;

use glam::{IVec4, Vec2, vec2};
use legion::storage::Component;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use sdl2::{pixels::Color, rect::FPoint};

#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
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

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicBody {
    pub handle: Option<RigidBodyHandle>,
    pub collider_handle: Option<ColliderHandle>,
    pub size: Vec2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StaticBody {
    pub handle: Option<RigidBodyHandle>,
    pub collider_handle: Option<ColliderHandle>,
    pub size: Vec2,
}

impl DynamicBody {
    pub fn new(size: Vec2) -> Self {
        DynamicBody {
            handle: None,
            collider_handle: None,
            size,
        }
    }
}

impl StaticBody {
    pub fn new(size: Vec2) -> Self {
        StaticBody {
            handle: None,
            collider_handle: None,
            size,
        }
    }
}
