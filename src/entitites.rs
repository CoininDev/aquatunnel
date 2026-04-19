use std::{collections::HashMap};

use crate::{
    comps::*,
    resources::{inventory::Inventory, weapons::DebugGun},
};
use egui_macroquad::egui;
use legion::World;
use macroquad::{
    color,
    math::{IVec2, Mat3, Vec2, vec2},
};

pub fn populate(world: &mut World) {
    //player
    let mut points: HashMap<String, Mat3> = HashMap::new();
    let weapon = Mat3::from_translation(Vec2::new(0.35, 0.));
    points.insert("weapon".to_string(), weapon);
    world.push((
        Transform {
            anchor_points: points,
            ..Default::default()
        },
        Sprite {
            image_path: "assets/diver1.png".into(),
            z_order: 5.5,
            flip_x: false,
            flip_y: false,
        },
        Player { speed: 100. },
        Body::new(Vec2::new(0.32 / 2., 0.32 / 2.), true),
        WeaponHolder {
            //weapon: Some(Box::new(Harpoon::default())),
            weapon: Some(Box::new(DebugGun {
                active: false,
                cooldown: 0.1,
            })),
        },
        InventoryHolder { inventory: Inventory::default() },
        //inventory window
        Window {
            title: "Inventory".into(),
            build_func: None
        }
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

    //window
    // world.push((
    //     Window {
    //         title: "Hello world".into(),
    //         build_func: Some(|ui| {
    //             let items = vec![
    //                 ("🗡️ Espada de Ferro", 1),
    //                 ("🛡️ Escudo de Madeira", 1),
    //                 ("🧪 Poção de Vida", 5),
    //                 ("🪙 Moedas de Ouro", 42),
    //             ];

    //             ui.label(egui::RichText::new("Itens").strong().size(16.0));
    //             ui.separator();

    //             egui::Grid::new("inventory_grid")
    //                 .num_columns(2)
    //                 .spacing([40.0, 6.0])
    //                 .striped(true)
    //                 .show(ui, |ui| {
    //                     ui.label(egui::RichText::new("Usar").underline());
    //                     ui.label(egui::RichText::new("Item").underline());
    //                     ui.label(egui::RichText::new("Qtd.").underline());
    //                     ui.end_row();

    //                     for (name, qty) in &items {
    //                         if ui.button("Usar").clicked() { println!("Usando {name}"); }
    //                         ui.label(*name);
    //                         ui.label(qty.to_string());
    //                         ui.end_row();
    //                     }
    //                 });

    //             ui.separator();
    //             ui.horizontal(|ui| {
    //                 ui.label("Total de itens:");
    //                 ui.label(
    //                     egui::RichText::new(items.len().to_string())
    //                         .strong()
    //                         .color(egui::Color32::GOLD),
    //                 );
    //             });

    //             ui.add_space(4.0);
    //             if ui.button("Fechar").clicked() {
    //                 // lógica para fechar/ocultar o inventário
    //             }
    //         })
    //     },
    // ));
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
