use std::any::Any;

use legion::systems::CommandBuffer;
use macroquad::{math::Vec2, time::get_frame_time};
use rapier2d::prelude::RigidBodyHandle;

use crate::{
    comps::{Body, Counter, Sprite, Transform, Weapon, WeaponHolder},
    resources::{physics::PhysicsContext, renderable::Renderable},
};

// COMPONENT
#[derive(Debug)]
pub struct Bullet {
    pub time_left: f32,
    pub velocity: Vec2,
}

impl Bullet {
    pub fn die(&self, ctx: &mut PhysicsContext, my_handle: RigidBodyHandle) {
        let mut bodies = ctx.bodies.borrow_mut();
        let mut islands = ctx.islands.borrow_mut();
        let mut colliders = ctx.colliders.borrow_mut();
        let mut impulse_joints = ctx.impulse_joints.borrow_mut();
        let mut multibody_joints = ctx.multibody_joints.borrow_mut();
        bodies.remove(
            my_handle,
            &mut islands,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            true,
        );
    }
}

#[derive(Debug, Clone)]
pub struct DebugGun {
    pub active: bool,
    pub cooldown: f32,
}

impl Weapon for DebugGun {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn is_active(&self) -> bool {
        self.active
    }

    fn image_path(&self) -> String {
        "assets/gun.png".into()
    }

    fn init(&self, cb: &mut CommandBuffer, _ctx: crate::comps::WeaponContext) {
        println!("Iniciando arma debug!");
        let mut w = _ctx.weapon_holder.weapon.as_ref().unwrap().box_clone();
        w.set_active(true);
        cb.add_component(
            _ctx.weapon_holder_entity,
            WeaponHolder {
                weapon: Some(w),
                .._ctx.weapon_holder.clone()
            },
        );
    }

    fn step(&self, cb: &mut CommandBuffer, _ctx: crate::comps::WeaponContext) {
        if self.cooldown > 0. {
            let mut w = _ctx.weapon_holder.weapon.as_ref().unwrap().box_clone();
            if let Some(debug_gun) = w.as_any_mut().downcast_mut::<DebugGun>() {
                debug_gun.cooldown -= get_frame_time();
            } else {
                eprintln!("Erro: arma não é DebugGun");
            }

            cb.add_component(
                _ctx.weapon_holder_entity,
                WeaponHolder {
                    weapon: Some(w),
                    .._ctx.weapon_holder.clone()
                },
            );
        }
    }

    fn shoot(&self, cb: &mut legion::systems::CommandBuffer, ctx: crate::comps::WeaponContext) {
        if self.cooldown > 0. {
            return;
        }

        let mut rigid_bodies = ctx.physics.unwrap().bodies.borrow_mut();
        let mut colliders = ctx.physics.unwrap().colliders.borrow_mut();

        let mut b = Body::new(Vec2::ONE * 0.05, true);
        let mut t = Transform {
            position: ctx.position,
            ..Default::default()
        };
        b.load(
            crate::comps::BodyType::Circle,
            &mut t,
            &mut rigid_bodies,
            &mut colliders,
        );
        cb.push((
            t,
            Bullet {
                velocity: Vec2::from_angle(ctx.rotation) * 4. * get_frame_time(),
                time_left: 6.,
            },
            Counter { times: 0 },
            b,
        ));

        let mut new_weapon = ctx.weapon_holder.weapon.as_ref().unwrap().box_clone();

        if let Some(debug_gun) = new_weapon.as_any_mut().downcast_mut::<DebugGun>() {
            debug_gun.cooldown = 0.3;
        } else {
            eprintln!("Erro: arma não é DebugGun");
        }

        // atualiza a entidade com a arma modificada
        cb.add_component(
            ctx.weapon_holder_entity,
            WeaponHolder {
                weapon: Some(new_weapon),
                ..ctx.weapon_holder.clone()
            },
        );
    }

    fn exit(&self, cb: &mut CommandBuffer, _ctx: crate::comps::WeaponContext) {
        println!("Bye bye!");
        let mut w = _ctx.weapon_holder.weapon.as_ref().unwrap().box_clone();
        w.set_active(false);
        cb.add_component(
            _ctx.weapon_holder_entity,
            WeaponHolder {
                weapon: Some(w),
                .._ctx.weapon_holder.clone()
            },
        );
    }

    fn box_clone(&self) -> Box<dyn Weapon> {
        Box::new(self.clone())
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Renderable for DebugGun {
    fn z_order(&self) -> f32 {
        40.
    }

    fn render(&self, transform: &crate::comps::Transform, textures: &crate::resources::Textures) {
        let mat = transform
            .global_mat_of_anchor_point("weapon")
            .expect("Missing 'weapon' anchor point");
        let transform = Transform::from_mat3(mat);

        let spr = Sprite {
            image_path: self.image_path(),
            z_order: self.z_order(),
            flip_x: false,
            flip_y: false,
        };

        spr.render(&transform, textures);
    }
}
