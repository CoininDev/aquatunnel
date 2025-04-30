use legion::{world::SubWorld, *};
use macroquad::{
    color::{self, *},
    math::*,
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    comps::{AnimationPlayer, DebugSprite, Player, Sprite, Spritesheet, Transform},
    game::{Time, Track},
};

#[system]
/// ```
pub fn clear_screen() {
    clear_background(DARKPURPLE);
}

#[system]
/// Draws the current frames per second (FPS) on the screen.
///
/// The FPS value is displayed as white text at the top-left corner of the screen.
///
/// # Examples
///
/// ```
/// draw_fps();
/// // Displays "FPS: <number>" at position (4, 24)
/// ```
pub fn draw_fps() {
    draw_text(format!("FPS: {}", get_fps()).as_str(), 4., 24., 24., WHITE);
}

#[system(for_each)]
/// Sets the z-order of a spritesheet to match the entity's y-position for depth sorting.
///
/// This ensures that entities are rendered in order based on their vertical position, creating a pseudo-3D layering effect.
///
/// # Examples
///
/// ```
/// let mut spritesheet = Spritesheet::default();
/// let transform = Transform { position: Vec2::new(0.0, 42.0), ..Default::default() };
/// z_y_axis_player(&mut spritesheet, &transform);
/// assert_eq!(spritesheet.z_order, 42.0);
/// ```
pub fn z_y_axis_player(spritesheet: &mut Spritesheet, transform: &Transform) {
    spritesheet.z_order = transform.position.y;
    //println!("{}", spritesheet.z_order);
}

/// Linearly interpolates between two values.
///
/// Returns a value between `from` and `to` based on the interpolation factor `t`, where `t` is typically between 0.0 and 1.0.
///
/// # Examples
///
/// ```
/// let result = lerp(0.0, 10.0, 0.5);
/// assert_eq!(result, 5.0);
/// ```
fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}
/// Linearly interpolates between two 2D vectors component-wise.
///
/// # Examples
///
/// ```
/// let a = Vec2::new(0.0, 0.0);
/// let b = Vec2::new(10.0, 20.0);
/// let result = lerp_vec2(a, b, 0.5);
/// assert_eq!(result, Vec2::new(5.0, 10.0));
/// ```
fn lerp_vec2(from: Vec2, to: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp(from.x, to.x, t), lerp(from.y, to.y, t))
}

const SMOOTHING_FACTOR: f32 = 10.0;
#[system]
/// Smoothly moves the camera target toward the tracked position and updates zoom based on screen size.
///
/// The camera's target is interpolated toward the tracked position using a smoothing factor and the frame's delta time.
/// The camera zoom is set to fit the screen dimensions, and the camera settings are applied.
///
/// # Examples
///
/// ```
/// camera(&time, &mut camera, &track);
/// ```
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
/// Resets the active camera to the default UI camera settings.
///
/// # Examples
///
/// ```
/// camera_ui();
/// // The camera is now set to the default UI configuration.
/// ```
pub fn camera_ui() {
    set_default_camera();
}

#[system(for_each)]
/// Updates the tracked position to match the player's current position.
///
/// Sets the `Track` resource's position to the player's transform position and prints the position to the console.
///
/// # Examples
///
/// ```
/// let mut track = Track { pos: Vec2::ZERO };
/// let player = Player {};
/// let transform = Transform { position: Vec2::new(10.0, 20.0), ..Default::default() };
/// track_player(&mut track, &player, &transform);
/// assert_eq!(track.pos, transform.position);
/// ```
pub fn track_player(#[resource] track: &mut Track, _: &Player, t: &Transform) {
    track.pos = t.position;
    println!("Player: {}", t.position);
}

// ---- RENDER SYSTEM ----
const METERS_TO_PIXELS: f32 = 100.0; /// Calculates a destination rectangle in pixel coordinates for rendering an entity.
///
/// Converts the entity's position and size from meters to pixels, applies scaling, and centers the rectangle on the position.
///
/// # Parameters
/// - `position`: The center position of the entity in meters.
/// - `size`: The size of the entity in meters.
/// - `scale`: The scaling factor to apply to the size.
///
/// # Returns
/// A `Rect` representing the destination rectangle in pixel coordinates, centered on the given position.
///
/// # Examples
///
/// ```
/// let position = Vec2::new(2.0, 3.0);
/// let size = Vec2::new(1.0, 1.0);
/// let scale = Vec2::new(1.0, 1.0);
/// let rect = calculate_dst(position, size, scale);
/// assert_eq!(rect.x, 150.0);
/// assert_eq!(rect.y, 250.0);
/// assert_eq!(rect.w, 100.0);
/// assert_eq!(rect.h, 100.0);
/// ```
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
    /// Returns the z-order value used for depth sorting during rendering.
    ///
    /// Entities with higher z-order values are rendered above those with lower values.
    ///
    /// # Examples
    ///
    /// ```
    /// let sprite = Sprite { z_order: 5.0, ..Default::default() };
    /// assert_eq!(sprite.z_order(), 5.0);
    /// ```
    fn z_order(&self) -> f32 {
        self.z_order
    }

    /// Renders the sprite at the given transform using the specified texture.
    ///
    /// Looks up the sprite's texture by image path, calculates its destination rectangle based on position, scale, and size, and draws it with rotation and centered pivot. If the texture is missing, the function prints an error and skips rendering.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut textures = HashMap::new();
    /// textures.insert("player.png".to_string(), Arc::new(load_texture("player.png").await.unwrap()));
    /// let sprite = Sprite { image_path: "player.png".to_string() };
    /// let transform = Transform::default();
    /// sprite.render(&transform, &textures);
    /// ```
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
    /// Returns the z-order value for sorting the renderable based on depth.
    ///
    /// Higher z-order values are rendered above lower ones.
    fn z_order(&self) -> f32 {
        self.0.z_order
    }

    /// Renders the current frame of an animated spritesheet at the specified transform.
    ///
    /// Looks up the appropriate texture and animation frame, calculates the destination rectangle,
    /// and draws the frame with scaling and rotation applied. If the texture or animation frame is missing,
    /// the function prints an error and skips rendering.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut textures = HashMap::new();
    /// textures.insert("player.png".to_string(), Arc::new(load_texture("player.png").await.unwrap()));
    /// let spritesheet = Spritesheet { /* ... */ };
    /// let anim_player = AnimationPlayer { /* ... */ };
    /// let transform = Transform { /* ... */ };
    /// (&spritesheet, &anim_player).render(&transform, &textures);
    /// ```
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

impl Renderable for DebugSprite {
    /// Returns the z-order value used for depth sorting during rendering.
    ///
    /// Entities with higher z-order values are rendered above those with lower values.
    fn z_order(&self) -> f32 {
        self.z_order
    }

    /// Renders a colored rectangle at the specified transform using the debug sprite's properties.
    ///
    /// # Examples
    ///
    /// ```
    /// let debug_sprite = DebugSprite { size: vec2(1.0, 1.0), color: RED };
    /// let transform = Transform::default();
    /// let textures = HashMap::new();
    /// debug_sprite.render(&transform, &textures);
    /// ```
    fn render(&self, transform: &Transform, textures: &HashMap<String, Arc<Texture2D>>) {
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
/// Renders all entities with renderable components, sorted by their z-order.
///
/// Collects sprites, animated sprites, and debug sprites from the ECS world, sorts them by their z-order (typically based on y-position for depth), and draws each using their associated transform and textures.
///
/// # Examples
///
/// ```
/// // In your game loop or system schedule:
/// render(&mut world, &textures);
/// ```
pub fn render(world: &mut SubWorld, #[resource] textures: &HashMap<String, Arc<Texture2D>>) {
    let mut renderables: Vec<(&Transform, &dyn Renderable)> = Vec::new();

    //Registering
    let mut sprite_query = <(&Sprite, &Transform)>::query();
    for (sprite, transform) in sprite_query.iter(world) {
        renderables.push((transform, sprite));
    }

    let mut animated_storage: Vec<(&Transform, (&Spritesheet, &AnimationPlayer))> = Vec::new();
    let mut animated_query = <(&Transform, &Spritesheet, &AnimationPlayer)>::query();
    for (t, s, p) in animated_query.iter(world) {
        animated_storage.push((t, (s, p)));
    }
    animated_storage
        .iter()
        .for_each(|(t, c)| renderables.push((t, c)));

    let mut debug_query = <(&Transform, &DebugSprite)>::query();
    for (transform, sprite) in debug_query.iter(world) {
        renderables.push((transform, sprite));
    }

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
