use legion::*;
use macroquad::{
    color::*, text::draw_text,
};

use crate::{
    comps::*,
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
pub fn debug_input(
    #[resource] ctx: &mut InputContext, 
    #[state] state: &mut bool
) {
    if ctx.consume_action(InputAction::DebugActionOn) {
        *state = true;
    }

    if ctx.consume_action(InputAction::DebugActionOff) {
        *state = false;
    }

    if *state {
        draw_text("Debug ativado", 24., 48., 24., WHITE);
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

#[system(for_each)]
pub fn load_uninitialized_bodies(
    #[resource] ctx: &mut PhysicsContext,
    transform: &mut Transform,
    body: &mut Body,
) {
    if body.body_handle.is_none() {
        body.load(
            crate::comps::BodyType::Rect,
            transform,
            &mut ctx.bodies,
            &mut ctx.colliders,
        );
    }
}

#[system]
pub fn step_physics(#[resource] p: &mut PhysicsContext) {
    p.pipeline.step(
        &p.gravity,
        &mut p.integration_parameters,
        &mut p.islands,
        &mut p.broad_phase,
        &mut p.narrow_phase,
        &mut p.bodies,
        &mut p.colliders,
        &mut p.impulse_joints,
        &mut p.multibody_joints,
        &mut p.ccd_solver,
        Some(&mut p.query_pipeline),
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

    let bodies = &ctx.bodies;
    if let Some(body) = bodies.get(body.body_handle.expect("Body não carregado")) {
        let pos = body.position().translation;
        transform.position.x = pos.x;
        transform.position.y = pos.y;
        //let rot = body.rotation().angle();
        //transform.rotation = rot;
    }
}
