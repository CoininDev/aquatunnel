use std::time::Instant;

use legion::{systems::CommandBuffer, world::SubWorld, *};
use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, rect::FPoint};

use crate::{
    comps::{DebugSprite, Player, Transform},
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
pub fn move_player(
    #[resource] input_ctx: &InputContext,
    tranform: &mut Transform,
    player: &Player,
) {
    tranform.position += input_ctx.move_direction * player.speed;
}
