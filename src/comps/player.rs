use std::fmt::Debug;

use legion::{systems::CommandBuffer, world::SubWorld, Entity};
use macroquad::math::Vec2;
use rapier2d::prelude::RigidBodyHandle;
#[derive(Debug)]
pub struct WeaponHolder {
    pub weapon: Box<dyn Weapon>,
    pub rotation: f32,
}


pub trait Weapon: Debug {
    fn image_path(&self) -> String;
    fn init(&mut self);
    fn step(&mut self);
    fn shoot(&mut self, e:Entity,  wh: &mut WeaponHolder, w: &SubWorld, cb:&mut CommandBuffer);
    fn exit(&mut self);
}

#[derive(Debug)]
struct Harpoon {
    blade_status: bool,
    blade_handle: RigidBodyHandle,
}
impl Weapon for Harpoon {
    fn image_path(&self) -> String {
        "assets/harpoon.png".into()
    }

    fn step(&mut self) {
        todo!("Caso estiver solto, mover blade.");
    }

    fn shoot(&mut self, e:Entity, wh: &mut WeaponHolder, w: &SubWorld, cb:&mut CommandBuffer) {
        if !self.blade_status {
            todo!("aplicar força para frente na blade");
            return;
        }

        todo!("aplicar força contrária ao player e resetar blade");
    }
    
    fn init(&mut self) {
        todo!()
    }
    
    fn exit(&mut self) {
        todo!()
    }
}