use legion::*;
use macroquad::{
    color,
    input::{is_key_down, is_mouse_button_down},
    math::Vec2,
    text::{draw_text, get_text_center},
};

use crate::{
    comps::{AnimationPlayer, Player, Spritesheet, Transform},
    game::Time,
    input::{InputContext, RawAction},
};

#[system]
/// Updates the input context by polling current key and mouse button states.
///
/// For each keybinding defined in the input context, checks if the corresponding key or mouse button is pressed.
/// If pressed, the associated action is added to the front of the input action queue.
///
/// # Examples
///
/// ```
/// // Assume `input_ctx` is a mutable InputContext resource.
/// input_update(&mut input_ctx);
/// // The input_ctx.actions queue is updated with any currently pressed actions.
/// ```
pub fn input_update(#[resource] input: &mut InputContext) {
    input.update();

    for (k, a) in input.setup.keybindings.clone() {
        match k {
            RawAction::Key(k) => {
                if is_key_down(k) {
                    input.actions.push_front(a);
                }
            }
            RawAction::MouseButton(b) => {
                if is_mouse_button_down(b) {
                    input.actions.push_front(a);
                }
            }
        }
    }
}

#[system(for_each)]
/// Advances the animation frame for an entity if its animation is playing.
///
/// Increments the animation frame based on elapsed time and loops back to the first frame when the end is reached. The frame timer is reset after each frame change.
///
/// # Examples
///
/// ```
/// let mut time = Time { delta: 0.2 };
/// let mut sprite_time = 0.0;
/// let mut player = AnimationPlayer {
///     playing: true,
///     frame_duration: 0.1,
///     current_animation: "walk".to_string(),
///     current_frame: 0,
/// };
/// let mut sheet = Spritesheet {
///     animations: [("walk".to_string(), vec![0, 1, 2])].iter().cloned().collect(),
/// };
/// step_animation(&mut time, &mut sprite_time, &mut player, &sheet);
/// assert_eq!(player.current_frame, 2);
/// ```
pub fn step_animation(
    #[resource] time: &mut Time,
    #[state] sprite_time: &mut f32,
    player: &mut AnimationPlayer,
    sheet: &Spritesheet,
) {
    if !player.playing {
        return;
    }

    if *sprite_time >= player.frame_duration {
        let anim_length = sheet
            .animations
            .get(player.current_animation.as_str())
            .unwrap()
            .len()
            - 1;

        if player.current_frame < anim_length {
            player.current_frame += 1;
        } else {
            player.current_frame = 0;
        }
        *sprite_time = 0.0;
    }
    *sprite_time += time.delta;
}

#[system(for_each)]
/// Updates the player's position based on input direction, speed, and elapsed time.
///
/// # Examples
///
/// ```
/// let mut input_ctx = InputContext { move_direction: Vec2::new(1.0, 0.0), ..Default::default() };
/// let time = Time { delta: 0.016 };
/// let mut transform = Transform { position: Vec2::ZERO, ..Default::default() };
/// let player = Player { speed: 100.0 };
/// move_player(&mut input_ctx, &time, &mut transform, &player);
/// assert_eq!(transform.position, Vec2::new(1.6, 0.0));
/// ```
pub fn move_player(
    #[resource] input_ctx: &mut InputContext,
    #[resource] time: &Time,
    transform: &mut Transform,
    player: &Player,
) {
    let dir = input_ctx.move_direction;
    transform.position += dir * player.speed * time.delta;
}

#[system(for_each)]
/// Renders a centered text at the player's position and updates the player's animation state based on movement direction.
///
/// The function draws the text "123" centered at the player's current position and sets the animation direction and playing state according to the input movement vector. If the player is stationary, the animation is set to "down" and paused; otherwise, it plays the appropriate directional animation.
///
/// # Examples
///
/// ```
/// // Within a Legion system:
/// animate_player(&mut input_ctx, &mut transform, &mut anim_player);
/// ```
pub fn animate_player(
    #[resource] input_ctx: &mut InputContext,
    transform: &mut Transform,
    anim_player: &mut AnimationPlayer,
) {
    let txt_center = get_text_center("123", None, 24, 1., 0.);
    draw_text(
        format!("123").as_str(),
        transform.position.x - txt_center.x,
        transform.position.y - txt_center.y,
        24.,
        color::GREEN,
    );
    match input_ctx.move_direction {
        Vec2 { x: 1.0, .. } => {
            anim_player.current_animation = "right".to_string();
            anim_player.playing = true;
        }
        Vec2 { x: -1.0, .. } => {
            anim_player.current_animation = "left".to_string();
            anim_player.playing = true;
        }
        Vec2 { y: 1.0, .. } => {
            anim_player.current_animation = "down".to_string();
            anim_player.playing = true;
        }
        Vec2 { y: -1.0, .. } => {
            anim_player.current_animation = "up".to_string();
            anim_player.playing = true;
        }
        Vec2 { x: 0.0, y: 0.0 } => {
            anim_player.current_animation = "down".to_string();
            anim_player.playing = false;
        }
        _ => {
            anim_player.playing = true;
        }
    }
}
