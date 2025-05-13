
use macroquad::math::{Vec2, vec2};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};


#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
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
