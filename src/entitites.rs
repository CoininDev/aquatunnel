use std::collections::HashMap;

use crate::comps::*;
use legion::World;
use macroquad::{
    color,
    math::{IVec4, Vec2},
};

pub fn populate(world: &mut World) {
    let player_anims = animations();
    //player
    world.push((
        Transform {
            position: Vec2::new(4.0, 0.0),
            ..Default::default()
        },
        Spritesheet {
            image_path: "assets/C3ZwL.png".to_string(),
            animations: player_anims,
            dst_size: Vec2::new(0.64, 0.64),
            z_order: 2.,
        },
        AnimationPlayer {
            current_animation: String::from("right"),
            current_frame: 0,
            playing: true,
            frame_duration: 0.1,
        },
        Player { speed: 0.7 },
        // DebugSprite {
        //     size: Vec2::new(0.32, 0.64),
        //     color: color::SKYBLUE,
        //     z_order: 0.1,
        // },
    ));

    //ground
    world.push((
        Transform {
            position: Vec2::new(4.0, 5.0),
            ..Default::default()
        },
        DebugSprite {
            size: Vec2::new(1.0, 1.0),
            color: color::WHITE,
            z_order: -1.,
        },
    ));
}

fn animations() -> HashMap<String, Vec<IVec4>> {
    let mut cu = HashMap::new();
    cu.insert(
        String::from("up"),
        vec![
            IVec4::new(64 * 0, 0, 64, 64),
            IVec4::new(64 * 1, 0, 64, 64),
            IVec4::new(64 * 2, 0, 64, 64),
            IVec4::new(64 * 3, 0, 64, 64),
            IVec4::new(64 * 4, 0, 64, 64),
            IVec4::new(64 * 5, 0, 64, 64),
            IVec4::new(64 * 6, 0, 64, 64),
            IVec4::new(64 * 7, 0, 64, 64),
            IVec4::new(64 * 8, 0, 64, 64),
        ],
    );
    cu.insert(
        String::from("left"),
        vec![
            IVec4::new(64 * 0, 64, 64, 64),
            IVec4::new(64 * 1, 64, 64, 64),
            IVec4::new(64 * 2, 64, 64, 64),
            IVec4::new(64 * 3, 64, 64, 64),
            IVec4::new(64 * 4, 64, 64, 64),
            IVec4::new(64 * 5, 64, 64, 64),
            IVec4::new(64 * 6, 64, 64, 64),
            IVec4::new(64 * 7, 64, 64, 64),
            IVec4::new(64 * 8, 64, 64, 64),
        ],
    );
    cu.insert(
        String::from("down"),
        vec![
            IVec4::new(64 * 0, 128, 64, 64),
            IVec4::new(64 * 1, 128, 64, 64),
            IVec4::new(64 * 2, 128, 64, 64),
            IVec4::new(64 * 3, 128, 64, 64),
            IVec4::new(64 * 4, 128, 64, 64),
            IVec4::new(64 * 5, 128, 64, 64),
            IVec4::new(64 * 6, 128, 64, 64),
            IVec4::new(64 * 7, 128, 64, 64),
            IVec4::new(64 * 8, 128, 64, 64),
        ],
    );
    cu.insert(
        String::from("right"),
        vec![
            IVec4::new(64 * 0, 192, 64, 64),
            IVec4::new(64 * 1, 192, 64, 64),
            IVec4::new(64 * 2, 192, 64, 64),
            IVec4::new(64 * 3, 192, 64, 64),
            IVec4::new(64 * 4, 192, 64, 64),
            IVec4::new(64 * 5, 192, 64, 64),
            IVec4::new(64 * 6, 192, 64, 64),
            IVec4::new(64 * 7, 192, 64, 64),
            IVec4::new(64 * 8, 192, 64, 64),
        ],
    );
    cu
}
