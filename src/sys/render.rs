use std::{
    collections::HashMap, sync::Arc
};
use legion::{world::SubWorld, *};
use macroquad::{*, math::*, color::*, window::*, prelude::*,};

use crate::comps::{AnimationPlayer, DebugSprite, DynamicBody, Sprite, Spritesheet, StaticBody, Transform};

#[system]
pub fn clear_screen() {
    clear_background(BLACK);
}

#[system]
pub fn draw_fps() {
    draw_text(
        format!("FPS: {}", get_fps()).as_str(), 
        0., 
        0., 
        12.,
        WHITE);
}

const METERS_TO_PIXELS: f32 = 100.0; // 1 metro = 100 pixels
fn calculate_dst(position:Vec2, size:Vec2, scale:Vec2) -> Rect {
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
#[read_component(DynamicBody)]
#[read_component(StaticBody)]
pub fn render(
    world: &mut SubWorld,
    #[resource] textures: &HashMap<String, Arc<Texture2D>>,
) {
    let mut sprite_query = <(&Sprite, &Transform)>::query();
    for (sprite, transform) in sprite_query.iter(world) {
        let texture = textures.get(sprite.image_path.as_str());
        let texture = match texture{
            Some(t) => t,
            None => {
                //eprintln!("Erro textura");
                return;
            }
        };
        let dst = calculate_dst(transform.position, Vec2::new(texture.width() as f32, texture.height() as f32), transform.scale);
        draw_texture_ex(
            &texture, 
            dst.x, 
            dst.y, 
            WHITE,
            DrawTextureParams{ 
                dest_size: Some(macroquad::math::Vec2::new(dst.w, dst.h)), 
                rotation: transform.rotation, 
                pivot: Some(macroquad::math::Vec2::new(dst.w/2., dst.h/2.)),
                ..Default::default()
            }
        );
    }

    // let mut debug_query = <(&DebugSprite, &Transform)>::query();
    // for (sprite, transform) in debug_query.iter_mut(world) {
    //     let sizex = sprite.size.x * transform.scale.x * METERS_TO_PIXELS;
    //     let sizey = sprite.size.y * transform.scale.y * METERS_TO_PIXELS;
    //     let px = transform.position.x * METERS_TO_PIXELS - (sizex / 2.0);
    //     let py = transform.position.y * METERS_TO_PIXELS - (sizey / 2.0);
    //     canvas.set_draw_color(sprite.color);
    //     canvas.draw_frect(FRect::new(px, py, sizex, sizey)).unwrap();
    // }

    let mut anim_query = <(&Transform, &Spritesheet, &AnimationPlayer)>::query();
    for (transform, spritesheet, player) in anim_query.iter_mut(world) {
        let texture = textures.get(spritesheet.image_path.as_str());
        let texture = match texture{
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
        // rect width and height are already in pixels format, so we need to revert to meters before passing it to calculate_dst
        let dst = calculate_dst(transform.position, spritesheet.dst_size, transform.scale);

        draw_texture_ex(
            &texture, 
            dst.x, 
            dst.y, 
            WHITE,
            DrawTextureParams{ 
                dest_size: Some(macroquad::math::Vec2::new(dst.w, dst.h)), 
                source: Some(rect),
                rotation: transform.rotation, 
                pivot: Some(macroquad::math::Vec2::new(dst.w/2., dst.h/2.)),
                ..Default::default()
            }
        );
    }

    // DEBUG
    let mut phys_query = <(&Transform, &DynamicBody)>::query();
    for (transform, body) in phys_query.iter(world) {
        let dst = calculate_dst(transform.position, body.size, Vec2::ONE);
        draw_rectangle_ex(
            dst.x, 
            dst.y, 
            dst.w, 
            dst.h, 
            DrawRectangleParams { 
                rotation: transform.rotation,
                color: color::SKYBLUE,
                ..Default::default()
            }
        );
    }

    let mut phys_query = <(&Transform, &StaticBody)>::query();
    for (transform, body) in phys_query.iter(world) {
        draw_rectangle_ex(
            transform.position.x, 
            transform.position.y, 
            body.size.x, 
            body.size.y, 
            DrawRectangleParams { 
                rotation: transform.rotation,
                color: color::MAGENTA,
                ..Default::default()
            }
        );
    }
}
