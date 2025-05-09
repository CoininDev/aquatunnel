use std::collections::HashMap;

use crate::comps::*;
use legion::World;
use macroquad::{
    color,
    math::{IVec2, IVec4, Vec2, vec2},
};

pub fn populate(world: &mut World) {
    let player_anims = animations();
    //player
    world.push((
        Transform {
            position: Vec2::new(4.0, 0.0),
            ..Default::default()
        },
        Sprite {
            image_path: "assets/diver1.png".into(),
            z_order: 5.5,
            flip_x: false,
            flip_y: false,
        },
        Player { speed: 2. },
        Body::new(Vec2::new(0.64 / 2., 0.64 / 2.), true),
    ));

    //block
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
        Body::new(Vec2::new(1.0 / 2., 1.0 / 2.), false),
    ));

    //tilemap
    world.push((
        Transform {
            scale: vec2(4., 4.),
            ..Default::default()
        },
        TileMap {
            tileset_path: "assets/dungeon_tiles.png".to_string(),
            z_order: 0.,
            tile_size: Vec2::new(8., 8.),
            tile_size_in_tileset: Vec2::new(8., 8.),
            tiles: tiles(),
        },
        //TileMapSource { matrix: matrix() },
    ));
}

fn matrix() -> Vec<Vec<u32>> {
    vec![
        vec![2, 3, 4, 3, 4, 3, 4, 3, 4, 5],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![6, 7, 8, 7, 8, 7, 8, 7, 8, 9],
    ]
}

fn tiles() -> HashMap<u32, IVec2> {
    let mut cu: HashMap<u32, IVec2> = HashMap::new();
    //plane
    cu.insert(1, IVec2::new(5, 5));
    //upperborders
    cu.extend((0..4).map(|i| (2 + i, IVec2::new((4 + i) as i32, 16))));
    //bottomborders
    cu.extend((0..4).map(|i| (6 + i, IVec2::new((4 + i) as i32, 17))));
    cu
}

fn animations() -> HashMap<String, Vec<IVec4>> {
    let mut cu = HashMap::new();
    cu.insert("up".to_string(), generate_frames(9, 0, 64));
    cu.insert("left".to_string(), generate_frames(9, 1, 64));
    cu.insert("down".to_string(), generate_frames(9, 2, 64));
    cu.insert("right".to_string(), generate_frames(9, 3, 64));
    cu
}

fn generate_frames(frame_quantity: i32, row: i32, frame_size: i32) -> Vec<IVec4> {
    (0..frame_quantity)
        .map(|i| IVec4::new(frame_size * i, frame_size * row, frame_size, frame_size))
        .collect()
}
