use std::collections::HashMap;

use crate::comps::*;
use legion::World;
use macroquad::{
    color,
    math::{IVec4, Vec2},
};

/// Populates the game world with player and ground entities, including their components and animations.
///
/// Adds a player entity with position, spritesheet, animation, and movement speed, as well as a ground entity with a debug sprite for visualization.
///
/// # Examples
///
/// ```
/// let mut world = World::default();
/// populate(&mut world);
/// // The world now contains a player and a ground entity.
/// ```
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

/// Returns a mapping of animation names to their frame rectangles for a spritesheet.
///
/// The returned hashmap contains four keys: "up", "left", "down", and "right". Each key maps to a vector of `IVec4` rectangles representing the pixel coordinates and size of each animation frame in the spritesheet. Each animation consists of 9 frames arranged horizontally, with each direction offset vertically by 64 pixels.
///
/// # Examples
///
/// ```
/// let anims = animations();
/// assert!(anims.contains_key("up"));
/// assert_eq!(anims["right"].len(), 9);
/// ```
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
