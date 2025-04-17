use crate::comps::*;
use legion::World;
use sdl2::{pixels::Color, rect::FPoint};

pub fn populate(world: &mut World) {
    world.push((
        Transform::default(),
        Sprite {
            image_path: "assets/diver1.png".to_string(),
        },
        DebugSprite {
            size: FPoint::new(40.0, 40.0),
            color: Color::CYAN,
        },
        Player { speed: 30.0 },
    ));
}
