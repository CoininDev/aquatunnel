use crate::comps::*;
use legion::World;
use sdl2::{pixels::Color, rect::FPoint};

pub fn populate(world: &mut World) {
    world.push((
        Transform::default(),
        DebugSprite {
            size: FPoint::new(40.0, 40.0),
            color: Color::CYAN,
        },
        Player { speed: 30.0 },
    ));
}
