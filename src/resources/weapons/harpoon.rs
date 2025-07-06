use std::cell::{Ref, RefMut};

use legion::{Entity, systems::CommandBuffer};
use macroquad::{
    color::{Color, GRAY, WHITE},
    math::{Vec2, vec2},
    texture::{DrawTextureParams, draw_texture_ex},
};
use nalgebra::{UnitComplex, vector};
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, NarrowPhase, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};

use crate::{
    comps::{Body, DebugSprite, Transform, Weapon, WeaponContext},
    resources::{
        METERS_TO_PIXELS,
        renderable::{Renderable, calculate_dst},
    },
};

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
    blade_entity: Entity,
}

impl Weapon for Harpoon {
    fn is_active(&self) -> bool {
        self.active
    }

    fn image_path(&self) -> String {
        "assets/harpoon gun.png".into()
    }

    fn init(&mut self, cb: &mut CommandBuffer, ctx: WeaponContext) {
        let Some(ph) = ctx.physics else {
            eprintln!("Erro> Harpoon exige acesso ao sistema de física");
            return;
        };
        let mut bodies = ph.bodies.borrow_mut();
        let mut colliders = ph.colliders.borrow_mut();

        let mut blade_b = Body::new(Vec2::ONE * 0.5, true);
        let mut blade_t = Transform {
            position: ctx.position,
            rotation: ctx.rotation,
            ..Default::default()
        };
        blade_b.load(
            crate::comps::BodyType::Rect,
            &mut blade_t,
            &mut bodies,
            &mut colliders,
        );
        let blade = cb.push((
            blade_t,
            blade_b,
            DebugSprite {
                z_order: 40.,
                color: GRAY,
                size: Vec2::ONE * 0.5,
            },
        ));
        self.blade_entity = blade;
        self.active = true;
    }

    fn step(&mut self, _cb: &mut CommandBuffer, ctx: WeaponContext) {
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

    fn exit(&mut self, _cb: &mut CommandBuffer, _ctx: WeaponContext) {
        self.active = false;
    }
}

impl Default for Harpoon {
    fn default() -> Self {
        Harpoon {
            blade_status: BladeStatus::Sleeping,
            active: false,
            last_hit: None,
        }
    }
}

impl Renderable for Harpoon {
    fn z_order(&self) -> f32 {
        0.0
    }
    // this transform, in this case will be from the player
    fn render(&self, transform: &Transform, textures: &crate::resources::Textures) {
        //DRAW WEAPON
        let tex = textures.0.get(&self.image_path());
        let tex = match tex {
            Some(t) => t,
            None => {
                eprintln!("Error getting texture (is harpoon weapon's image_path wrong?)");
                return;
            }
        };
        let transform_mat = transform
            .global_mat_of_anchor_point("weapon")
            .expect("Player doesn't have the 'weapon' anchor point.");
        let weapon_transform: Transform = Transform::from_mat3(transform_mat);
        let dst = calculate_dst(
            weapon_transform.position,
            Vec2::new(tex.width(), tex.height()) / METERS_TO_PIXELS,
            weapon_transform.scale,
        );
        draw_texture_ex(
            &tex,
            dst.x,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(dst.w, dst.h)),
                rotation: weapon_transform.rotation,
                ..Default::default()
            },
        );
        //DRAW LINE

        //DRAW BLADE
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
        //TODO
    }
}
