use std::time::Instant;

use legion::{systems::CommandBuffer, *};
use sdl2::{pixels::Color, rect::FPoint};

use crate::{
    comps::{DebugSprite, Transform},
    game::Time,
};

#[system]
pub fn delta_update(#[resource] time: &mut Time) {
    let now = Instant::now();
    time.delta = now.duration_since(time.last).as_secs_f32();
    time.last = now;
}
#[system]
pub fn spawn(cmd: &mut CommandBuffer, #[state] counter: &mut u32) {
    if *counter <= 70000 {
        cmd.push((
            Transform::default(),
            DebugSprite {
                size: FPoint::new(80.0, 80.0),
                color: Color::MAGENTA,
            },
        ));
        println!("{}", *counter);
        *counter += 1;
    }
}

#[system(for_each)]
pub fn move_squares(transform: &mut Transform) {
    transform.position.x += 1.0;
    transform.position.y += 1.0;
}

