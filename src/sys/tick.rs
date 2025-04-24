use std::time::Instant;
use legion::*;
use macroquad::{input::{is_key_down, is_mouse_button_down}, math::Vec2};
use nalgebra::Vector2;

use crate::{
    comps::{AnimationPlayer, DynamicBody, Player, Spritesheet, Transform},
    game::Time,
    input::{InputContext, RawAction},
    physics::PhysicsContext,
};

#[system]
pub fn time_update(#[resource] time: &mut Time) {
    let now = Instant::now();
    time.delta = now.duration_since(time.last).as_secs_f32();
    time.last = now;
}

#[system]
pub fn input_update(#[resource] input: &mut InputContext) {
    input.update();

    for (k, a) in input.setup.keybindings.clone() {
        match k {
            RawAction::Key(k) => {
                if is_key_down(k){ input.actions.push_front(a); }
            },
            RawAction::MouseButton(b) => {
                if is_mouse_button_down(b) { input.actions.push_front(a); }
            }
        }
    }
}

#[system(for_each)]
pub fn step_animation(
    #[resource] time: &Time,
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
pub fn step_physics(#[resource] physics: &mut PhysicsContext) {
    physics.pipeline.step(
        &physics.gravity,
        &physics.integration_params,
        &mut physics.islands,
        &mut physics.broad_phase,
        &mut physics.narrow_phase,
        &mut physics.bodies,
        &mut physics.colliders,
        &mut physics.impulse_joints,
        &mut physics.multibody_joints,
        &mut physics.ccd_solver,
        None,
        &mut physics.hooks,
        &mut physics.events,
    );
}

#[system(for_each)]
pub fn physics_integration(
    #[resource] physics: &mut PhysicsContext,
    transform: &mut Transform,
    body: &DynamicBody,
) {
    if let Some(b) = physics.bodies.get(body.handle.unwrap()) {
        let t = b.translation();
        let r = b.rotation();
        transform.position = Vec2::new(t.x, t.y);
        transform.rotation = r.angle();
    }
}

#[system(for_each)]
pub fn move_player(
    #[resource] input_ctx: &InputContext,
    //#[resource] time: &Time,
    #[resource] physics: &mut PhysicsContext,
    //transform: &mut Transform,
    anim_player: &mut AnimationPlayer,
    body: &mut DynamicBody,
    player: &Player,
) {
    let dir = input_ctx.move_direction;

    // --- Atualiza a velocidade do corpo no mundo físico ---
    if let Some(rigid_body) = physics.bodies.get_mut(body.handle.unwrap()) {
        let velocity = Vector2::new(dir.x * player.speed, dir.y * player.speed);
        if velocity.magnitude() != 0.0 {
            rigid_body.set_linvel(velocity, true);
        }
    }
    match input_ctx.move_direction {
        Vec2 { x: 1.0, .. } => {
            anim_player.current_animation = "right".to_string();
            anim_player.playing = true;
        }
        Vec2 { x: -1.0, .. } => {
            anim_player.current_animation = "left".to_string();
            anim_player.playing = true;
        }
        Vec2 { y: 1.0, .. } => {
            anim_player.current_animation = "down".to_string();
            anim_player.playing = true;
        }
        Vec2 { y: -1.0, .. } => {
            anim_player.current_animation = "up".to_string();
            anim_player.playing = true;
        }
        Vec2 { x: 0.0, y: 0.0 } => {
            anim_player.current_animation = "down".to_string();
            anim_player.playing = false;
        }
        _ => {
            anim_player.playing = true;
        }
    }
}
