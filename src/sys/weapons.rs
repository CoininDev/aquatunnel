use legion::{Entity, World, query::*, system, systems::CommandBuffer, world::SubWorld};
use macroquad::time::get_frame_time;
use nalgebra::{Matrix, Vector, Vector2};

use crate::{
    comps::*,
    resources::{
        input::{InputAction, InputContext},
        physics::PhysicsContext,
        weapons::Bullet,
    },
};

fn get_weapon_transform(player_transform: &Transform) -> Transform {
    let w_mat = player_transform
        .global_mat_of_anchor_point("weapon")
        .expect("player não tem weapon anchor point");
    Transform::from_mat3(w_mat)
}

#[system]
#[write_component(Body)]
#[write_component(Counter)]
#[write_component(Bullet)]
pub fn bullet_spawn(
    world: &mut SubWorld,
    #[resource] pc: &mut PhysicsContext,
    cb: &mut CommandBuffer,
) {
    let mut query = <(Entity, &mut Bullet, &mut Body, &mut Counter)>::query();
    for (e, bullet, body, counter) in query.iter_mut(world) {
        println!("cu");
        if counter.times <= 0 {
            let mut bodies = pc.bodies.borrow_mut();
            if let Some(rb) = bodies.get_mut(body.body_handle.unwrap()) {
                let impulse = bullet.velocity;
                let impulse = Vector2::new(impulse.x, impulse.y);
                rb.apply_impulse(impulse, true);
            }
            counter.times += 1;
        } else {
            bullet.time_left -= get_frame_time();
            if bullet.time_left <= 0. {
                bullet.die(pc, body.body_handle.unwrap());
                cb.remove(*e);
            }
        }
    }
}

#[system]
#[read_component(WeaponHolder)]
#[read_component(Transform)]
#[read_component(Body)]
pub fn init_weapons(world: &SubWorld, #[resource] pc: &mut PhysicsContext, cb: &mut CommandBuffer) {
    let player = <(Entity, &Body, &Transform, &WeaponHolder)>::query()
        .iter(world)
        .next();

    let Some((e, b, t, w)) = player else {
        return;
    };

    if let Some(weapon) = w.weapon.as_ref() {
        if !weapon.is_active() {
            let w_t = get_weapon_transform(t);
            let ctx = WeaponContext {
                weapon_holder: w,
                weapon_holder_entity: *e,
                player_body: &b,
                world,
                rotation: w_t.rotation,
                position: w_t.position,
                physics: Some(pc),
            };
            weapon.init(cb, ctx);
        }
    }
}

#[system]
#[read_component(WeaponHolder)]
#[read_component(Transform)]
#[read_component(Body)]
pub fn shoot(
    world: &SubWorld,
    #[resource] pc: &mut PhysicsContext,
    #[resource] ic: &mut InputContext,
    cb: &mut CommandBuffer,
) {
    let player = <(Entity, &Body, &Transform, &WeaponHolder)>::query()
        .iter(world)
        .next();

    let Some((e, b, t, w)) = player else {
        return;
    };

    let w_t = get_weapon_transform(t);
    let ctx = WeaponContext {
        weapon_holder: w,
        weapon_holder_entity: *e,
        world: world,
        player_body: b,
        rotation: w_t.rotation,
        position: w_t.position,
        physics: Some(pc),
    };

    if let Some(weapon) = w.weapon.as_ref() {
        if ic.consume_action(InputAction::DebugActionOn) {
            weapon.shoot(cb, ctx);
        }
    }
}

#[system]
#[read_component(WeaponHolder)]
#[read_component(Transform)]
#[read_component(Body)]
pub fn step(
    world: &SubWorld,
    #[resource] pc: &mut PhysicsContext,
    //#[resource] ic: &mut InputContext,
    cb: &mut CommandBuffer,
) {
    let player = <(Entity, &Body, &Transform, &WeaponHolder)>::query()
        .iter(world)
        .next();

    let Some((e, b, t, w)) = player else {
        return;
    };

    let w_t = get_weapon_transform(t);
    let ctx = WeaponContext {
        weapon_holder: w,
        weapon_holder_entity: *e,
        world: world,
        player_body: b,
        rotation: w_t.rotation,
        position: w_t.position,
        physics: Some(pc),
    };

    if let Some(weapon) = w.weapon.as_ref() {
        weapon.step(cb, ctx);
    }
}

#[system]
#[read_component(WeaponHolder)]
#[read_component(Transform)]
#[read_component(Body)]
pub fn step(
    world: &SubWorld,
    #[resource] pc: &mut PhysicsContext,
    //#[resource] ic: &mut InputContext,
    cb: &mut CommandBuffer,
) {
    let player = <(Entity, &Body, &Transform, &WeaponHolder)>::query()
        .iter(world)
        .next();

    let Some((e, b, t, w)) = player else {
        return;
    };

    let w_t = get_weapon_transform(t);
    let ctx = WeaponContext {
        weapon_holder: w,
        weapon_holder_entity: *e,
        world: world,
        player_body: b,
        rotation: w_t.rotation,
        position: w_t.position,
        physics: Some(pc),
    };

    if let Some(weapon) = w.weapon.as_ref() {
        weapon.step(cb, ctx);
    }
}
