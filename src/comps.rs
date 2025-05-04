use macroquad::{
    color::Color,
    math::{IVec2, IVec4, Vec2, vec2},
};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
pub struct TileMap {
    pub tileset_path: String,
    pub tiles: HashMap<u32, IVec2>,
    pub tile_size: Vec2,
    pub z_order: f32,
}

/// TileMap can be used with local TileMapSource, or alternately based on external info, as chunkmanager
#[derive(Debug, Clone, PartialEq)]
pub struct TileMapSource {
    pub matrix: Vec<Vec<u32>>,
}
