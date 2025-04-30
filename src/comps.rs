use macroquad::{
    color::Color,
    math::{IVec4, Vec2, vec2},
};
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
    /// Returns a `Transform` with position at the origin, unit scale, and zero rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = Transform::default();
    /// assert_eq!(t.position, vec2(0.0, 0.0));
    /// assert_eq!(t.scale, vec2(1.0, 1.0));
    /// assert_eq!(t.rotation, 0.0);
    /// ```
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
