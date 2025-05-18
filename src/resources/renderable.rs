use std::fmt::Debug;

use macroquad::color::*;
use macroquad::math::*;
use macroquad::shapes::*;
use macroquad::texture::*;

use crate::comps::*;

use super::Textures;

pub const METERS_TO_PIXELS: f32 = 100.0; // 1 metro = 100 pixels

fn calculate_dst(position: Vec2, size: Vec2, scale: Vec2) -> Rect {
    // Corrigindo os cálculos de tamanho
    let sizex = size.x * scale.x * METERS_TO_PIXELS;
    let sizey = size.y * scale.y * METERS_TO_PIXELS;

    // Corrigindo os cálculos de posição
    let px = (position.x * METERS_TO_PIXELS) - (sizex / 2.0);
    let py = (position.y * METERS_TO_PIXELS) - (sizey / 2.0);

    Rect::new(px, py, sizex, sizey)
}

pub trait Renderable: Debug {
    fn z_order(&self) -> f32;
    fn render(&self, transform: &Transform, textures: &Textures);
}

impl Renderable for Sprite {
    fn z_order(&self) -> f32 {
        self.z_order
    }

    fn render(&self, transform: &Transform, textures: &Textures) {
        let texture = textures.0.get(self.image_path.as_str());
        let texture = match texture {
            Some(t) => t,
            _ => {
                eprintln!("Erro textura");
                return;
            }
        };
        let dst = calculate_dst(
            transform.position,
            Vec2::new(texture.width() as f32, texture.height() as f32) / METERS_TO_PIXELS,
            transform.scale,
        );
        draw_texture_ex(
            &texture,
            dst.x,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(dst.w, dst.h)),
                rotation: transform.rotation,
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                ..Default::default()
            },
        );
    }
}

impl Renderable for (&Spritesheet, &AnimationPlayer) {
    fn z_order(&self) -> f32 {
        self.0.z_order
    }

    fn render(&self, transform: &Transform, textures: &Textures) {
        let texture = textures.0.get(self.0.image_path.as_str());
        let texture = match texture {
            Some(t) => t,
            _ => {
                eprintln!("Erro textura");
                return;
            }
        };

        let rect = self
            .0
            .animations
            .get(self.1.current_animation.as_str())
            .expect("Animation not found")
            .get(self.1.current_frame)
            .expect("Animation frame out of bounds");
        let rect = Rect::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.z as f32);
        let dst = calculate_dst(transform.position, self.0.dst_size, transform.scale);

        draw_texture_ex(
            &texture,
            dst.x,
            dst.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(dst.w, dst.h)),
                source: Some(rect),
                rotation: transform.rotation,
                pivot: Some(macroquad::math::Vec2::new(dst.w / 2., dst.h / 2.)),
                ..Default::default()
            },
        );
    }
}

impl Renderable for (&TileMap, &TileMapSource) {
    fn z_order(&self) -> f32 {
        self.0.z_order
    }

    fn render(&self, transform: &Transform, textures: &Textures) {
        let tilemap = self.0;
        let source = self.1;
        for y in 0..source.matrix.len() {
            for x in 0..source.matrix[0].len() {
                let tile_id = source.matrix[y as usize][x as usize];
                let world_x =
                    x as f32 * tilemap.tile_size.x * transform.scale.x + transform.position.x;
                let world_y =
                    y as f32 * tilemap.tile_size.y * transform.scale.y + transform.position.y;
                let src = tilemap
                    .tiles
                    .get(&tile_id)
                    .expect("Algum tile não corresponde aos Tiles conhecidos");
                let src_rect = Rect::new(
                    (src.x as f32 * tilemap.tile_size.x) as f32,
                    (src.y as f32 * tilemap.tile_size.y) as f32,
                    tilemap.tile_size.x as f32,
                    tilemap.tile_size.y as f32,
                );
                draw_texture_ex(
                    textures
                        .0
                        .get(&tilemap.tileset_path)
                        .expect("Tileset não carregada"),
                    world_x,
                    world_y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(src_rect),
                        dest_size: Some(vec2(
                            tilemap.tile_size.x * transform.scale.x,
                            tilemap.tile_size.y * transform.scale.y,
                        )),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

impl Renderable for (&TileMap, &Chunk) {
    fn z_order(&self) -> f32 {
        self.0.z_order
    }

    fn render(&self, transform: &Transform, textures: &Textures) {
        let tilemap = self.0;
        let source = self.1;
        if source.matrix.is_none() {
            return;
        }
        let matrix = source.matrix.as_ref().unwrap();
        for y in 0..matrix.height {
            for x in 0..matrix.width {
                let tile_id = matrix.get(x, y).unwrap();
                let screen_x =
                    x as f32 * tilemap.tile_size.x * METERS_TO_PIXELS * transform.scale.x;
                //+ transform.position.x;

                let screen_y =
                    y as f32 * tilemap.tile_size.y * METERS_TO_PIXELS * transform.scale.y;
                //+ transform.position.y;
                let src = tilemap
                    .tiles
                    .get(tile_id)
                    .expect("Algum tile não corresponde aos Tiles conhecidos");
                let src_rect = Rect::new(
                    (src.x as f32 * tilemap.tile_size_in_tileset.x) as f32,
                    (src.y as f32 * tilemap.tile_size_in_tileset.y) as f32,
                    tilemap.tile_size_in_tileset.x as f32,
                    tilemap.tile_size_in_tileset.y as f32,
                );
                draw_texture_ex(
                    textures
                        .0
                        .get(&tilemap.tileset_path)
                        .expect("Tileset não carregada"),
                    screen_x + (transform.position.x * METERS_TO_PIXELS),
                    screen_y + (transform.position.y * METERS_TO_PIXELS),
                    WHITE,
                    DrawTextureParams {
                        source: Some(src_rect),
                        dest_size: Some(vec2(
                            tilemap.tile_size.x * METERS_TO_PIXELS * transform.scale.x,
                            tilemap.tile_size.y * METERS_TO_PIXELS * transform.scale.y,
                        )),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

impl Renderable for DebugSprite {
    fn z_order(&self) -> f32 {
        self.z_order
    }

    fn render(&self, transform: &Transform, _: &Textures) {
        let dst = calculate_dst(transform.position, self.size, transform.scale);
        draw_rectangle_ex(
            dst.x,
            dst.y,
            dst.w,
            dst.h,
            DrawRectangleParams {
                offset: Vec2::ZERO,
                rotation: transform.rotation,
                color: self.color,
            },
        );
    }
}

impl Renderable for Body {
    fn z_order(&self) -> f32 {
        0.
    }

    fn render(&self, transform: &Transform, _: &Textures) {
        let dst = calculate_dst(transform.position, self.size, transform.scale * 2.);
        draw_rectangle_ex(
            dst.x,
            dst.y,
            dst.w,
            dst.h,
            DrawRectangleParams {
                offset: Vec2::ZERO,
                color: colors::RED,
                ..Default::default()
            },
        );
    }
}

impl Renderable for WeaponHolder {
    fn z_order(&self) -> f32 {
        60.
    }

    fn render(&self, transform: &Transform, textures: &Textures) {
        let dst = calculate_dst(transform.position, vec2(0.7, 0.7), transform.scale);
        if let Some(weapon) = self.weapon.as_ref() {
            let texture = textures.0.get(weapon.image_path().as_str());
            let texture = match texture {
                Some(t) => t,
                _ => {
                    eprintln!("Erro textura");
                    return;
                }
            };
            draw_texture_ex(
                &texture,
                dst.x,
                dst.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(dst.w, dst.h)),
                    rotation: transform.rotation,
                    flip_x: false,
                    flip_y: false,
                    ..Default::default()
                },
            );
        }
    }
}
