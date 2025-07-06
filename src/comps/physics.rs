use macroquad::math::Vec2;
use nalgebra::vector;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, ColliderSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};

use super::Transform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyType {
    Circle,
    Rect,
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

    pub fn load(
        &mut self,
        body: BodyType,
        transform: &mut Transform,
        rigid_bodies: &mut RigidBodySet,
        colliders: &mut ColliderSet,
    ) {
        let mut rb = RigidBodyBuilder::dynamic()
            .translation(vector![transform.position.x, transform.position.y])
            .build();
        if !self.is_dynamic {
            rb = RigidBodyBuilder::fixed()
                .translation(vector![transform.position.x, transform.position.y])
                .build();
        }
        let col = match body {
            BodyType::Circle => ColliderBuilder::ball(self.size.x).build(),
            BodyType::Rect => ColliderBuilder::cuboid(self.size.x, self.size.y).build(),
            _ => {
                eprintln!("Erro: função load não suporta esse tipo de corpo");
                return;
            }
        };
        self.body_handle = Some(rigid_bodies.insert(rb));
        self.collider_handle =
            Some(colliders.insert_with_parent(col, self.body_handle.unwrap(), rigid_bodies));
    }
}
