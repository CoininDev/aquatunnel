use legion::*;
use macroquad::{
    color::{self, WHITE},
    math::Vec2,
    text::{draw_text, get_text_center}, time::get_frame_time,
};
use nalgebra::vector;

use crate::{
    comps::{AnimationPlayer, Body, Player, Sprite, Spritesheet, Transform},
    resources::{
        Time,
        input::{InputAction, InputContext},
        physics::PhysicsContext,
    },
};

#[system]
pub fn input_update(#[resource] input: &mut InputContext) {
    input.update();
}

#[system]
pub fn debug_input(#[resource] ctx: &mut InputContext, #[state] state: &mut bool) {
    if ctx.consume_action(InputAction::DebugActionOn) {
        *state = true;
    }

    if ctx.consume_action(InputAction::DebugActionOff) {
        *state = false;
    }

    if *state {
        draw_text("Bunda", 24., 48., 24., WHITE);
    }
}

#[system(for_each)]
pub fn step_animation(
    #[resource] time: &mut Time,
    #[state] sprite_time: &mut f32,
    player: &mut AnimationPlayer,
    sheet: &Spritesheet,
) {
    if !player.playing {
        return;
    }

    if *sprite_time >= player.frame_duration {
        let anim_length = sheet
            .animations
            .get(player.current_animation.as_str())
            .unwrap()
            .len()
            - 1;

        if player.current_frame < anim_length {
            player.current_frame += 1;
        } else {
            player.current_frame = 0;
        }
        *sprite_time = 0.0;
    }
    *sprite_time += time.delta;
}

#[system]
pub fn step_physics(#[resource] p: &mut PhysicsContext) {
    let gravity = p.gravity;
    let mut integration_parameters = p.integration_parameters.borrow_mut();
    let mut islands = p.islands.borrow_mut();
    let mut broad_phase = p.broad_phase.borrow_mut();
    let mut narrow_phase = p.narrow_phase.borrow_mut();
    let mut bodies = p.bodies.borrow_mut();
    let mut colliders = p.colliders.borrow_mut();
    let mut impulse_joints = p.impulse_joints.borrow_mut();
    let mut multibody_joints = p.multibody_joints.borrow_mut();
    let mut ccd_solver = p.ccd_solver.borrow_mut();
    let mut query_pipeline = p.query_pipeline.borrow_mut();

    p.pipeline.borrow_mut().step(
        &gravity,
        &mut *integration_parameters,
        &mut *islands,
        &mut *broad_phase,
        &mut *narrow_phase,
        &mut *bodies,
        &mut *colliders,
        &mut *impulse_joints,
        &mut *multibody_joints,
        &mut *ccd_solver,
        Some(&mut *query_pipeline),
        &(),
        &(),
    );
}

#[system(for_each)]
pub fn integrate_physics(
    #[resource] ctx: &mut PhysicsContext,
    transform: &mut Transform,
    body: &mut Body,
) {
    if !body.is_dynamic {
        return;
    }

    let bodies = ctx.bodies.borrow();
    if let Some(body) = bodies.get(body.body_handle.expect("Body não carregado")) {
        let pos = body.position().translation;
        transform.position.x = pos.x;
        transform.position.y = pos.y;
        //let rot = body.rotation().angle();
        //transform.rotation = rot;
    }
}
