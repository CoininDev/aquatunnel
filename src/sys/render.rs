use legion::{world::SubWorld, *};
use macroquad::{color::*, math::*, prelude::*};
use std::{cmp, collections::HashMap, sync::Arc};

use crate::comps::{AnimationPlayer, DebugSprite, Sprite, Spritesheet, Transform};

#[system]
pub fn clear_screen() {
    clear_background(DARKPURPLE);
}

#[system]
pub fn draw_fps() {
    draw_text(format!("FPS: {}", get_fps()).as_str(), 4., 24., 24., WHITE);
}

#[system(for_each)]
pub fn z_y_axis_player(spritesheet: &mut Spritesheet, transform: &Transform) {
    spritesheet.z_order = transform.position.y;
    println!("{}", spritesheet.z_order);
}

// ---- RENDER SYSTEM ----

enum RenderComp {
    Sprite(Sprite),
    AnimatedSprite(Spritesheet, AnimationPlayer),
    DebugSprite(DebugSprite),
}

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
#[system]
#[read_component(Sprite)]
#[read_component(Transform)]
#[read_component(DebugSprite)]
#[read_component(Spritesheet)]
#[read_component(AnimationPlayer)]
pub fn render(world: &mut SubWorld, #[resource] textures: &HashMap<String, Arc<Texture2D>>) {
    let mut renderables: Vec<(Transform, RenderComp)> = Vec::new();

    //Registering
    let mut sprite_query = <(&Sprite, &Transform)>::query();
    for (sprite, transform) in sprite_query.iter(world) {
        renderables.push((transform.clone(), RenderComp::Sprite(sprite.clone())));
    }
    let mut animated_query = <(&Transform, &Spritesheet, &AnimationPlayer)>::query();
    for (transform, spritesheet, player) in animated_query.iter(world) {
        renderables.push((
            transform.clone(),
            RenderComp::AnimatedSprite(spritesheet.clone(), player.clone()),
        ));
    }
    let mut debug_query = <(&Transform, &DebugSprite)>::query();
    for (transform, sprite) in debug_query.iter(world) {
        renderables.push((transform.clone(), RenderComp::DebugSprite(sprite.clone())));
    }

    renderables.sort_by(|a, b| {
        let (_, x) = a;
        let az: f32 = match x {
            RenderComp::Sprite(s) => s.z_order,
            RenderComp::DebugSprite(s) => s.z_order,
            RenderComp::AnimatedSprite(s, _) => s.z_order,
        };
        let (_, x) = b;
        let bz: f32 = match x {
            RenderComp::Sprite(s) => s.z_order,
            RenderComp::DebugSprite(s) => s.z_order,
            RenderComp::AnimatedSprite(s, _) => s.z_order,
        };

        az.total_cmp(&bz)
    });

    //Rendering
    for renderable in renderables.iter() {
        let (transform, comp) = renderable;
        match comp {
            RenderComp::Sprite(sprite) => {
                let texture = textures.get(sprite.image_path.as_str());
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
            RenderComp::AnimatedSprite(spritesheet, player) => {
                let texture = textures.get(spritesheet.image_path.as_str());
                let texture = match texture {
                    Some(t) => t,
                    None => {
                        //eprintln!("Erro textura");
                        return;
                    }
                };

                let rect = spritesheet
                    .animations
                    .get(player.current_animation.as_str())
                    .expect("Animation not found")
                    .get(player.current_frame)
                    .expect("Animation frame out of bounds");
                let rect = Rect::new(rect.x as f32, rect.y as f32, rect.w as f32, rect.z as f32);
                let dst = calculate_dst(transform.position, spritesheet.dst_size, transform.scale);

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
            RenderComp::DebugSprite(sprite) => {
                let dst = calculate_dst(transform.position, sprite.size, transform.scale);
                //println!("a{:?}", dst);
                draw_rectangle(dst.x, dst.y, dst.w, dst.h, sprite.color);
                draw_circle(dst.x + (dst.w / 2.), dst.y + (dst.h / 2.), 4., BLACK);
            }
        }
    }
}
