use std::{any::Any, fmt::Debug};

use legion::{Entity, World, systems::CommandBuffer, world::SubWorld};
use macroquad::math::Vec2;

use crate::resources::{physics::PhysicsContext, renderable::Renderable};

use super::Body;

#[derive(Debug, Clone)]
pub struct WeaponHolder {
    pub weapon: Option<Box<dyn Weapon>>,
}

pub trait Weapon: Debug + Send + Sync + Renderable + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn image_path(&self) -> String;
    fn init(&self, cb: &mut CommandBuffer, ctx: WeaponContext);
    fn step(&self, cb: &mut CommandBuffer, ctx: WeaponContext);
    fn shoot(&self, cb: &mut CommandBuffer, ctx: WeaponContext);
    fn exit(&self, cb: &mut CommandBuffer, ctx: WeaponContext);

    fn box_clone(&self) -> Box<dyn Weapon>;
}
impl Clone for Box<dyn Weapon> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
#[derive(Clone)]
pub struct WeaponContext<'a> {
    pub weapon_holder: &'a WeaponHolder,
    pub weapon_holder_entity: Entity,
    pub player_body: &'a Body,
    pub world: &'a SubWorld<'a>,
    pub rotation: f32,
    pub position: Vec2,
    pub physics: Option<&'a PhysicsContext>,
}
