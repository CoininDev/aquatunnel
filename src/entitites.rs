use std::collections::HashMap;

use crate::comps::*;
use legion::World;
use macroquad::{
    color,
    math::{IVec2, Vec2, vec2},
};

pub fn populate(world: &mut World) {
    //player
    world.push((
        Transform::default(),
        Sprite {
            image_path: "assets/diver1.png".into(),
            z_order: 5.5,
            flip_x: false,
            flip_y: false,
        },
        Player { speed: 100. },
        Body::new(Vec2::new(0.32 / 2., 0.32 / 2.), true),
        WeaponHolder {
            weapon: Some(Box::new(Harpoon::default())),
        },
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
fn tiles() -> HashMap<u32, IVec2> {
    let mut cu: HashMap<u32, IVec2> = HashMap::new();
    //plane
    cu.insert(1, IVec2::new(5, 5));
    //upperborders
    //cu.extend((0..4).map(|i| (2 + i, IVec2::new((4 + i) as i32, 16))));
    //bottomborders
    //cu.extend((0..4).map(|i| (6 + i, IVec2::new((4 + i) as i32, 17))));
    cu
}
