use legion::{world::SubWorld, *};
use macroquad::{color::*, math::*, prelude::*};
use std::{collections::HashMap, sync::Arc};

use crate::{
    comps::{
        AnimationPlayer, DebugSprite, Player, Sprite, Spritesheet, TileMap, TileMapSource,
        Transform,
    },
    game::{Time, Track},
};

#[system]
pub fn clear_screen() {
    clear_background(DARKPURPLE);
}

#[system]
pub fn draw_fps() {
    #[cfg(debug_assertions)]
    draw_text(format!("FPS: {}", get_fps()).as_str(), 4., 24., 24., WHITE);
}

#[system(for_each)]
pub fn z_y_axis_player(spritesheet: &mut Spritesheet, transform: &Transform) {
    spritesheet.z_order = transform.position.y;
    //println!("{}", spritesheet.z_order);
}

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}
fn lerp_vec2(from: Vec2, to: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp(from.x, to.x, t), lerp(from.y, to.y, t))
}

const SMOOTHING_FACTOR: f32 = 10.0;
#[system]
pub fn camera(
    #[resource] time: &Time,
    #[resource] camera: &mut Box<Camera2D>,
    #[resource] track: &Track,
) {
    camera.target = lerp_vec2(
        camera.target,
        track.pos * METERS_TO_PIXELS,
        time.delta * SMOOTHING_FACTOR,
    );
    camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
    set_camera(camera.as_ref());
}

#[system]
pub fn camera_ui() {
    set_default_camera();
}

#[system(for_each)]
pub fn track_player(#[resource] track: &mut Track, _: &Player, t: &Transform) {
    track.pos = t.position;
}

// ---- RENDER SYSTEM ----
const METERS_TO_PIXELS: f32 = 100.0; // 1 metro = 100 pixels
fn calculate_dst(position: Vec2, size: Vec2, scale: Vec2) -> Rect {
    // Corrigindo os cálculos de tamanho
    let sizex = size.x * scale.x * METERS_TO_PIXELS;
    let sizey = size.y * scale.y * METERS_TO_PIXELS;

    // Corrigindo os cálculos de posição
    let px = (position.x * METERS_TO_PIXELS) - (sizex / 2.0);
    let py = (position.y * METERS_TO_PIXELS) - (sizey / 2.0);

    Rect::new(px, py, sizex, sizey)
}

trait Renderable {
    fn z_order(&self) -> f32;
    fn render(&self, transform: &Transform, textures: &HashMap<String, Arc<Texture2D>>);
}

impl Renderable for Sprite {
    fn z_order(&self) -> f32 {
        self.z_order
    }

    fn render(&self, transform: &Transform, textures: &HashMap<String, Arc<Texture2D>>) {
        let texture = textures.get(self.image_path.as_str());
        let texture = match texture {
            Some(t) => t,
            None => {
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
                pivot: Some(macroquad::math::Vec2::new(dst.w / 2., dst.h / 2.)),
                ..Default::default()
            },
        );
    }
}

impl Renderable for (&Spritesheet, &AnimationPlayer) {
    fn z_order(&self) -> f32 {
        self.0.z_order
    }

    fn render(&self, transform: &Transform, textures: &HashMap<String, Arc<Texture2D>>) {
        let texture = textures.get(self.0.image_path.as_str());
        let texture = match texture {
            Some(t) => t,
            None => {
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

    fn render(&self, transform: &Transform, textures: &HashMap<String, Arc<Texture2D>>) {
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

impl Renderable for DebugSprite {
    fn z_order(&self) -> f32 {
        self.z_order
    }

    fn render(&self, transform: &Transform, _: &HashMap<String, Arc<Texture2D>>) {
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

#[system]
#[read_component(Sprite)]
#[read_component(Transform)]
#[read_component(DebugSprite)]
#[read_component(Spritesheet)]
#[read_component(AnimationPlayer)]
#[read_component(TileMap)]
#[read_component(TileMapSource)]
pub fn render(world: &mut SubWorld, #[resource] textures: &HashMap<String, Arc<Texture2D>>) {
    let mut renderables: Vec<(&Transform, &dyn Renderable)> = Vec::new();

    //Registering
    let mut sprite_query = <(&Sprite, &Transform)>::query();
    for (sprite, transform) in sprite_query.iter(world) {
        renderables.push((transform, sprite));
    }

    let animated_storage: Vec<_> = <(&Transform, &Spritesheet, &AnimationPlayer)>::query()
        .iter(world)
        .map(|(t, s, p)| (t, (s, p)))
        .collect();

    animated_storage
        .iter()
        .for_each(|(t, c)| renderables.push((t, c)));

    let mut debug_query = <(&Transform, &DebugSprite)>::query();
    for (transform, sprite) in debug_query.iter(world) {
        renderables.push((transform, sprite));
    }

    let tile_storage: Vec<_> = <(&Transform, &TileMap, &TileMapSource)>::query()
        .iter(world)
        .map(|(t, m, s)| (t, (m, s)))
        .collect();

    tile_storage
        .iter()
        .for_each(|(t, c)| renderables.push((t, c)));

    renderables.sort_by(|a, b| {
        let (_, x) = a;
        let az = x.z_order();
        let (_, x) = b;
        let bz = x.z_order();

        az.total_cmp(&bz)
    });

    //Rendering
    for renderable in renderables.iter() {
        let (transform, comp) = renderable;
        comp.render(&transform, &textures);
    }
}
