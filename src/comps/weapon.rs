use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};

use legion::{Entity, systems::CommandBuffer};
use macroquad::math::{Vec2, vec2};
use nalgebra::{UnitComplex, vector};
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, NarrowPhase, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};

use crate::resources::physics::PhysicsContext;

use super::Body;

#[derive(Debug)]
pub struct WeaponHolder {
    pub weapon: Option<Box<dyn Weapon>>,
}

pub trait Weapon: Debug + Send + Sync {
    fn is_active(&self) -> bool;
    fn image_path(&self) -> String;
    fn init(&mut self, ctx: WeaponContext);
    fn step(&mut self, ctx: WeaponContext);
    fn shoot(&mut self, cb: &mut CommandBuffer, ctx: WeaponContext);
    fn exit(&mut self, ctx: WeaponContext);
}

#[derive(Clone)]
pub struct WeaponContext<'a> {
    pub player_body: &'a Body,
    pub rotation: f32,
    pub position: Vec2,
    pub physics: Option<&'a PhysicsContext>,
}

pub fn surface_type_to_bit(s: SurfaceType) -> u128 {
    s as u128
}

#[derive(Debug)]
enum BladeStatus {
    Sleeping,
    Running,
    Fixed,
}
#[derive(Debug)]
pub enum SurfaceType {
    None,
    Wall,
    Monster,
    Item,
}

#[derive(Debug)]
enum SurfaceHit {
    Wall,
    Monster(Entity),
    Item(Entity),
}

#[derive(Debug)]
pub struct Harpoon {
    active: bool,
    blade_status: BladeStatus,
    last_hit: Option<SurfaceHit>,
    pub blade_handle: Option<RigidBodyHandle>,
    blade_col_handle: Option<ColliderHandle>,
}
impl Weapon for Harpoon {
    fn image_path(&self) -> String {
        "assets/harpoon gun.png".into()
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn init(&mut self, ctx: WeaponContext) {
        let Some(ph) = ctx.physics else {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        };
        let mut bodies = ph.bodies.borrow_mut();
        let mut colliders = ph.colliders.borrow_mut();

        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![ctx.position.x, ctx.position.y])
            .build();
        let col = ColliderBuilder::ball(0.08).sensor(true).build();

        self.blade_handle = Some(bodies.insert(rb));
        self.blade_col_handle =
            Some(colliders.insert_with_parent(col, self.blade_handle.unwrap(), &mut bodies));
        self.active = true;
    }

    fn step(&mut self, ctx: WeaponContext) {
        let Some(ph) = ctx.physics else {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        };
        let narrow = ph.narrow_phase.borrow();
        if self._is_blade_colliding(narrow) {
            self._fix(ctx.clone());
        }

        match self.blade_status {
            BladeStatus::Sleeping => self._step_sleeping(ctx),
            BladeStatus::Running => {}
            BladeStatus::Fixed => {}
        }
    }

    fn shoot(&mut self, _cb: &mut CommandBuffer, ctx: WeaponContext) {
        if ctx.physics.is_none() {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        }

        match self.blade_status {
            BladeStatus::Sleeping => self._shoot(ctx),
            BladeStatus::Running | BladeStatus::Fixed => self._retract(ctx),
        }
    }

    fn exit(&mut self, _ctx: WeaponContext) {
        self.active = false;
    }
}

impl Default for Harpoon {
    fn default() -> Self {
        Harpoon {
            blade_status: BladeStatus::Sleeping,
            active: false,
            blade_handle: None,
            blade_col_handle: None,
            last_hit: None,
        }
    }
}

const SHOOT_FORCE: f32 = 10.;
impl Harpoon {
    fn _step_sleeping(&mut self, ctx: WeaponContext) {
        let ph = ctx.physics.unwrap();
        let mut bodies = ph.bodies.borrow_mut();
        if let Some(rb) = bodies.get_mut(self.blade_handle.unwrap()) {
            rb.set_rotation(UnitComplex::from_angle(ctx.rotation), true);
        }
    }

    fn _is_blade_colliding(&self, narrow: Ref<NarrowPhase>) -> bool {
        narrow
            .contact_pairs_with(self.blade_col_handle.unwrap())
            .next()
            .is_some()
    }

    fn _shoot(&mut self, ctx: WeaponContext) {
        self.blade_status = BladeStatus::Running;
        let ph = ctx.physics.unwrap();
        let mut bodies = ph.bodies.borrow_mut();
        if let Some(rb) = bodies.get_mut(self.blade_handle.unwrap()) {
            let target = ctx.position - (Vec2::from_angle(ctx.rotation) * SHOOT_FORCE);
            rb.add_force(vector![target.x, target.y], true);
        }
    }

    fn _retract(&mut self, ctx: WeaponContext) {
        let ph = ctx.physics.unwrap();
        let mut bodies = ph.bodies.borrow_mut();
        let player_handle = ctx.player_body.body_handle;

        let blade_pos = if let Some(rb) = bodies.get_mut(self.blade_handle.unwrap()) {
            rb.lock_translations(false, true);
            let p = rb.position().translation;
            vec2(p.x, p.y)
        } else {
            vec2(0., 0.)
        };

        if let Some(hit) = &self.last_hit {
            match hit {
                SurfaceHit::Wall => {
                    self._apply_retract_force_wall(bodies, player_handle.unwrap(), blade_pos)
                }

                _ => {}
            }
        }
    }

    fn _fix(&mut self, ctx: WeaponContext) {
        let ph = ctx.physics.unwrap();
        let mut bodies = ph.bodies.borrow_mut();
        let colliders = ph.colliders.borrow();
        let narrow = ph.narrow_phase.borrow();

        if let Some(rb) = bodies.get_mut(self.blade_handle.unwrap()) {
            rb.lock_translations(true, true);
        }

        for contact_pair in narrow.contact_pairs_with(self.blade_col_handle.unwrap()) {
            let other = if contact_pair.collider1 == self.blade_col_handle.unwrap() {
                contact_pair.collider2
            } else {
                contact_pair.collider1
            };
            let Some(other_collider) = colliders.get(other) else {
                continue;
            };
            match other_collider.user_data {
                0 => continue,
                1 => {
                    self.last_hit = Some(SurfaceHit::Wall);
                    break;
                }
                _ => {
                    //TODO: Detect monsters and items
                }
            }
        }
    }

    fn _apply_retract_force_wall(
        &self,
        mut bodies: RefMut<RigidBodySet>,
        player_handle: RigidBodyHandle,
        blade_pos: Vec2,
    ) {
        if let Some(rb) = bodies.get_mut(player_handle) {
            rb.add_force(vector![blade_pos.x, blade_pos.y], true);
        }
    }

    fn _apply_retract_force_entity(
        &self,
        mut bodies: RefMut<RigidBodySet>,
        player_handle: RigidBodyHandle,
        blade_pos: Vec2,
        entity: Entity,
    ) {
    }
}
