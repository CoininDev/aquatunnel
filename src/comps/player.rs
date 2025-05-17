use std::{cell::RefMut, fmt::Debug, vec};

use legion::{systems::CommandBuffer, world::SubWorld, Entity, EntityStore};
use macroquad::{math::{vec2, Vec2}, window::set_fullscreen};
use nalgebra::{vector, UnitComplex};
use rapier2d::prelude::{ColliderBuilder, ColliderSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};

use crate::resources::physics::PhysicsContext;

use super::Body;

#[derive(Debug)]
pub struct WeaponHolder {
    pub weapon: Box<dyn Weapon>
}


pub trait Weapon: Debug {
    fn image_path(&self) -> String;
    fn init(&mut self, ctx:WeaponContext);
    fn step(&mut self, ctx:WeaponContext);
    fn shoot(&mut self, w: &SubWorld, cb:&mut CommandBuffer, ctx:WeaponContext);
    fn exit(&mut self, ctx:WeaponContext);
}


pub struct WeaponContext<'a> {
    pub player: Entity,
    pub rotation: f32,
    pub position: Vec2,
    pub physics: Option<&'a mut PhysicsContext>
}

#[derive(Debug)]
pub struct Harpoon {
    blade_status: BladeStatus,
    pub blade_handle: RigidBodyHandle,
}
#[derive(Debug)]
enum BladeStatus {
    Sleeping,
    Running,
    Fixed,
}

impl Weapon for Harpoon {
    fn image_path(&self) -> String {
        "assets/harpoon.png".into()
    }
    
    fn init(&mut self, ctx:WeaponContext) {
        if ctx.physics.is_none() {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        }

        let ph = ctx.physics.unwrap();
        let mut bodies = ph.bodies.borrow_mut();
        let mut colliders = ph.colliders.borrow_mut();

        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![ctx.position.x, ctx.position.y])
            .build();
        let col = ColliderBuilder::ball(0.08)
            .sensor(true)  
            .build();

        self.blade_handle = bodies.insert(rb);
        colliders.insert_with_parent(col, self.blade_handle, &mut bodies);
    }
    
    fn step(&mut self, ctx:WeaponContext) {
        if ctx.physics.is_none() {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        }

        self._check_collision(&ctx);

        match self.blade_status {
            BladeStatus::Sleeping => self._step_sleeping(ctx),
            BladeStatus::Running => self._step_running(ctx),
            BladeStatus::Fixed => self._step_fixed(ctx),
        }
    }


    
    fn shoot(&mut self, w: &SubWorld, cb:&mut CommandBuffer, ctx:WeaponContext) {
        if ctx.physics.is_none() {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        }

        match self.blade_status {
            BladeStatus::Sleeping => self._shoot(ctx),
            BladeStatus::Running | BladeStatus::Fixed => self._retract(w, ctx),
        }
    }
    
    fn exit(&mut self, ctx:WeaponContext) {
        todo!()
    }

}


const SHOOT_FORCE:f32 = 10.;
impl Harpoon {
    fn _step_sleeping(&mut self, ctx:WeaponContext) {
        let ph = ctx.physics.unwrap();
        let mut bodies =  ph.bodies.borrow_mut();
        if let Some(rb) = bodies.get_mut(self.blade_handle) {
            rb.set_rotation(UnitComplex::from_angle(ctx.rotation), true);
        }
    }

    fn _step_running(&mut self, _ctx:WeaponContext) {
        {}
    }

    fn _step_fixed(&mut self, ctx:WeaponContext) {
        let ph = ctx.physics.unwrap();
        let mut bodies =  ph.bodies.borrow_mut();
        if let Some(rb) = bodies.get_mut(self.blade_handle) {
            rb.lock_translations(true, true);
        }
    }

    fn _check_collision(&mut self, ctx: &WeaponContext){
        todo!();
    }

    fn _shoot(&mut self, ctx:WeaponContext){
        self.blade_status = BladeStatus::Running;
        let ph = ctx.physics.unwrap();
        let mut bodies =  ph.bodies.borrow_mut();
        if let Some(rb) = bodies.get_mut(self.blade_handle) {
            let target = ctx.position - (Vec2::from_angle(ctx.rotation) * SHOOT_FORCE);
            rb.add_force(vector![target.x, target.y], true);
        }
    }

    fn _retract(&mut self, w:&SubWorld, ctx:WeaponContext){
        let ph = ctx.physics.unwrap();
        let mut bodies =  ph.bodies.borrow_mut();
        let player_handle = 
        if let Ok(cu) = w.entry_ref(ctx.player) {
            Some(cu.get_component::<Body>().expect("Erro> Player não possui corpo").body_handle)
        } else {
            eprintln!("Player não existe");
            return;
        };

        let blade_pos = if let Some(rb) = bodies.get(self.blade_handle) {
            let p = rb.position().translation;
            vec2(p.x, p.y)
        } else {
            vec2(0.,0.)
        };

        if let Some(rb) = bodies.get_mut(player_handle.unwrap().unwrap()) {
            rb.add_force(vector![blade_pos.x, blade_pos.y], true);
        }
    }
}