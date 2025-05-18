use legion::{query::*, system, systems::CommandBuffer, world::SubWorld};

use crate::{
    comps::*,
    resources::{
        input::{InputAction, InputContext},
        physics::PhysicsContext,
    },
};

#[system]
#[write_component(WeaponHolder)]
#[read_component(Transform)]
#[write_component(Body)]
pub fn init_weapons(world: &mut SubWorld, #[resource] pc: &mut PhysicsContext) {
    let player = <(&mut Body, &Transform, &mut WeaponHolder)>::query()
        .iter_mut(world)
        .next();

    let Some((b, t, w)) = player else {
        return;
    };

    if let Some(weapon) = w.weapon.as_mut() {
        if !weapon.is_active() {
            let ctx = WeaponContext {
                player_body: &b,
                rotation: t.rotation,
                position: t.position,
                physics: Some(pc),
            };
            weapon.init(ctx);
        }
    }
}

#[system]
#[write_component(WeaponHolder)]
#[read_component(Transform)]
#[write_component(Body)]
pub fn shoot(
    world: &mut SubWorld,
    #[resource] pc: &mut PhysicsContext,
    #[resource] ic: &mut InputContext,
    cb: &mut CommandBuffer,
) {
    let player = <(&mut Body, &Transform, &mut WeaponHolder)>::query()
        .iter_mut(world)
        .next();

    let Some(player) = player else {
        return;
    };

    let ctx = WeaponContext {
        player_body: player.0,
        rotation: player.1.rotation,
        position: player.1.position,
        physics: Some(pc),
    };

    if let Some(weapon) = player.2.weapon.as_mut() {
        if ic.consume_action(InputAction::DebugActionOn) {
            weapon.shoot(cb, ctx);
        }
    }
}
