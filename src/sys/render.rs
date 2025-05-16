use legion::{world::SubWorld, *};
use macroquad::{color::*, math::*, prelude::*};

use crate::{
    comps::{
        AnimationPlayer, Body, Chunk, DebugSprite, Player, Sprite, Spritesheet, TileMap, TileMapSource, Transform
    },
    resources::{renderable::Renderable, *},
};

#[system]
pub fn clear_screen() {
    clear_background(DARKBLUE);
}

#[system]
pub fn draw_fps() {
    //#[cfg(debug_assertions)]
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

#[system]
#[read_component(Sprite)]
#[read_component(Transform)]
#[read_component(DebugSprite)]
#[read_component(Spritesheet)]
#[read_component(AnimationPlayer)]
#[read_component(TileMap)]
#[read_component(TileMapSource)]
#[read_component(Body)]
#[read_component(Chunk)]
pub fn render(world: &mut SubWorld, #[resource] textures: &Textures) {
    let mut renderables: Vec<(&Transform, &dyn Renderable)> = Vec::new();

    //Registering
    <(&Transform, &Sprite)>::query()
        .iter(world)
        .for_each(|(t, s)| renderables.push((t, s)));

    //Here, for keeping borrows safe, we need to collect before registering
    let animated_sotage = <(&Transform, &Spritesheet, &AnimationPlayer)>::query()
        .iter(world)
        .map(|(t, s, p)| (t, (s, p)))
        .collect::<Vec<_>>();
    animated_sotage
        .iter()
        .for_each(|(t, r)| renderables.push((t, r)));

    <(&Transform, &DebugSprite)>::query()
        .iter(world)
        .for_each(|(t, r)| renderables.push((t, r)));

    let tilemap_storage = <(&Transform, &TileMap, &TileMapSource)>::query()
        .iter(world)
        .map(|(t, m, s)| (t, (m, s)))
        .collect::<Vec<_>>();
    tilemap_storage
        .iter()
        .for_each(|(t, r)| renderables.push((t, r)));

    let chunk_storage = <(&Transform, &TileMap, &Chunk)>::query()
        .iter(world)
        .map(|(t, m, c)| (t, (m, c)))
        .collect::<Vec<_>>();
    chunk_storage
        .iter()
        .for_each(|(t, r)| renderables.push((t, r)));

    #[cfg(debug_assertions)]
    <(&Transform, &Body)>::query()
        .iter(world)
        .for_each(|(t,b)| renderables.push((t, b)));

    //Sorting
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
        //println!("{:?}", comp);
    }
}
