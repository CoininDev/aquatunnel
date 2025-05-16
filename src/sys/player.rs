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



#[system(for_each)]
pub fn move_player(
    #[resource] input_ctx: &mut InputContext,
    #[resource] physics_ctx: &mut PhysicsContext,
    player: &Player,
    transform: &mut Transform,
    sprite: &mut Sprite,
    body: &Body,
) {
    let mut bodies = physics_ctx.bodies.borrow_mut();
    if let Some(rb) = bodies.get_mut(body.body_handle.expect("Body não carregado")) {
        let dir = input_ctx.move_direction;
        let velocity = dir * player.speed * get_frame_time();
        rb.set_linvel(vector![velocity.x, velocity.y], true);
    }
    sprite.flip_x = input_ctx.look_direction.x < 0.;
    transform.rotation = if input_ctx.look_direction.x < 0. {
        input_ctx.look_direction.to_angle() + std::f32::consts::PI
    } else {
        input_ctx.look_direction.to_angle()
    };
}

#[system(for_each)]
pub fn animate_player(
    #[resource] input_ctx: &mut InputContext,
    transform: &mut Transform,
    anim_player: &mut AnimationPlayer,
) {
    let txt_center = get_text_center("123", None, 24, 1., 0.);
    draw_text(
        format!("123").as_str(),
        transform.position.x - txt_center.x,
        transform.position.y - txt_center.y,
        24.,
        color::GREEN,
    );
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
