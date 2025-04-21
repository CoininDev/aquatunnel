use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use glam::Vec2;
use legion::{world::SubWorld, *};
use sdl2::{
    pixels::Color,
    rect::{FRect, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Sdl2TtfContext,
    video::WindowContext,
};

use crate::comps::{AnimationPlayer, DebugSprite, DynamicBody, Sprite, Spritesheet, StaticBody, Transform};

#[system]
pub fn clear_screen(#[resource] canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
}

#[system]
pub fn draw_fps(
    #[resource] canvas: &mut WindowCanvas,
    #[resource] texture_creator: &mut TextureCreator<WindowContext>,
    #[resource] ttf_ctx: &Arc<Sdl2TtfContext>,
    #[state] frames: &mut u32,
    #[state] fps: &mut String,
    #[state] last_time: &mut Instant,
) {
    let elapsed = last_time.elapsed();
    //println!("{}", elapsed.as_micros());
    if elapsed >= Duration::from_secs(1) {
        *fps = frames.clone().to_string();
        *frames = 0;
        *last_time = Instant::now();
    }

    let font = ttf_ctx
        .load_font("/usr/share/fonts/ubuntu/Ubuntu-M.ttf", 12)
        .unwrap();
    let surface = font
        .render(format!("FPS: {}", fps).as_str())
        .blended(Color::WHITE)
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    let dst = Rect::new(0, 0, texture.query().width, texture.query().height);
    canvas.copy(&texture, None, Some(dst)).unwrap();
    *frames += 1;
}

const METERS_TO_PIXELS: f32 = 50.0; // 1 metro = 100 pixels
fn calculate_dst(position:Vec2, size:Vec2, scale:Vec2) -> FRect {
    // Corrigindo os cálculos de tamanho
    let sizex = size.x * scale.x * METERS_TO_PIXELS;
    let sizey = size.y * scale.y * METERS_TO_PIXELS;

    // Corrigindo os cálculos de posição
    let px = (position.x * METERS_TO_PIXELS) - (sizex / 2.0);
    let py = (position.y * METERS_TO_PIXELS) - (sizey / 2.0);
    
    FRect::new(px, py, sizex, sizey)
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
    #[resource] canvas: &mut WindowCanvas,
    #[resource] textures: &HashMap<String, Arc<Texture<'_>>>,
) {
    let mut sprite_query = <(&Sprite, &Transform)>::query();
    for (sprite, transform) in sprite_query.iter(world) {
        let texture = textures.get(sprite.image_path.as_str()).unwrap();
        let texquery = texture.query();
        let dst = calculate_dst(transform.position, Vec2::new(texquery.width as f32, texquery.height as f32), transform.scale);
        canvas
            .copy_ex_f(
                texture.as_ref(),
                None,
                dst,
                transform.rotation.into(),
                None,
                false,
                false,
            )
            .unwrap();
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

    // let mut anim_query = <(&Transform, &Spritesheet, &AnimationPlayer)>::query();
    // for (transform, spritesheet, player) in anim_query.iter_mut(world) {
    //     let tex = textures.get(spritesheet.image_path.as_str());
    //     let tex = tex.unwrap();

    //     let rect = spritesheet
    //         .animations
    //         .get(player.current_animation.as_str())
    //         .expect(
    //             format!(
    //                 "The animation '{}' does not exist in spritesheet.",
    //                 player.current_animation,
    //             )
    //             .as_str(),
    //         )
    //         .get(player.current_frame)
    //         .expect(
    //             format!(
    //                 "The position {} in animation {} is out of bounds.",
    //                 player.current_frame, player.current_animation
    //             )
    //             .as_str(),
    //         );
    //     // rect width and height are already in pixels format, so we need to revert to meters before passing it to calculate_dst
    //     let dst = calculate_dst(transform.position, Vec2::new(rect.w as f32 / METERS_TO_PIXELS, rect.z as f32 / METERS_TO_PIXELS), transform.scale);

    //     canvas
    //         .copy_ex_f(
    //             tex.as_ref(),
    //             Some(Rect::new(rect.x, rect.y, rect.w as u32, rect.z as u32)),
    //             dst,
    //             transform.rotation.into(),
    //             None,
    //             false,
    //             false,
    //         )
    //         .unwrap();
    // }

    // DEBUG
    let mut phys_query = <(&Transform, &DynamicBody)>::query();
    for (transform, body) in phys_query.iter(world) {
        canvas.set_draw_color(Color::CYAN);
        canvas.draw_frect(calculate_dst(transform.position, Vec2::new(body.size.x, body.size.y), transform.scale)).unwrap();
    }

    let mut phys_query = <(&Transform, &StaticBody)>::query();
    for (transform, body) in phys_query.iter(world) {
        canvas.set_draw_color(Color::MAGENTA);
        canvas.draw_frect(calculate_dst(transform.position, Vec2::new(body.size.x, body.size.y), transform.scale)).unwrap();
    }
} 

#[system]
pub fn present(#[resource] canvas: &mut WindowCanvas) {
    canvas.present();
}
