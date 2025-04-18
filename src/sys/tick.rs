use std::time::Instant;

use glam::Vec2;
use legion::{systems::CommandBuffer, world::SubWorld, *};
use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::FPoint};

use crate::{
    comps::{AnimationPlayer, DebugSprite, Player, Spritesheet, Transform},
    game::Time,
    input::InputContext,
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

#[system(for_each)]
pub fn move_player(
    #[resource] input_ctx: &InputContext,
    #[resource] time: &Time,
    transform: &mut Transform,
    spritesheet: &Spritesheet,
    anim_player: &mut AnimationPlayer,
    player: &Player,
) {
    transform.position += input_ctx.move_direction * player.speed * time.delta;

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
