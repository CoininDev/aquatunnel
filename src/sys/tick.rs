use legion::*;
use macroquad::{
    color,
    input::{is_key_down, is_mouse_button_down},
    math::Vec2,
    text::{draw_text, get_text_center},
};
use nalgebra::Vector2;
use std::time::Instant;

use crate::{
    comps::{AnimationPlayer, DynamicBody, Player, Spritesheet, Transform},
    game::Time,
    input::{InputContext, RawAction},
    physics::PhysicsContext,
};

#[system]
pub fn input_update(#[resource] input: &mut InputContext) {
    input.update();

    for (k, a) in input.setup.keybindings.clone() {
        match k {
            RawAction::Key(k) => {
                if is_key_down(k) {
                    input.actions.push_front(a);
                }
            }
            RawAction::MouseButton(b) => {
                if is_mouse_button_down(b) {
                    input.actions.push_front(a);
                }
            }
        }
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
pub fn move_player(
    #[resource] input_ctx: &mut InputContext,
    transform: &mut Transform,
    player: &Player,
) {
    let dir = input_ctx.move_direction;
    transform.position += dir * player.speed;
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
        transform.position.y - txt_center.y - 30.,
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
