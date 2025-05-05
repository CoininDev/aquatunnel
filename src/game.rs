use std::{collections::HashMap, sync::Arc};

use legion::{Resources, Schedule, World, systems::CommandBuffer};
use macroquad::{
    camera::Camera2D,
    input::{KeyCode, is_key_down},
    math::Vec2,
    texture::Texture2D,
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    comps::Transform,
    entitites::populate,
    input::{InputContext, InputSetup},
    load::{load, physics_load},
    physics,
    resources::*,
    sys::*,
};
pub async fn run_game() -> Result<(), String> {
    let mut world = World::default();
    let mut resources = Resources::default();

    //Registering physics resources
    physics::init_physics(&mut resources);

    resources.insert(Track { pos: Vec2::ZERO });
    resources.insert(Textures(HashMap::new()));
    resources.insert(InputContext::new(InputSetup::default()));
    resources.insert(Box::new(Camera2D::default()));
    resources.insert(RenderQueue(Vec::new()));
    resources.insert(CommandBuffer::new(&world));

    populate(&mut world);

    // Systems involving macroquad rendering or input requires local thread
    let mut step_schedule = Schedule::builder()
        .add_thread_local(tick::input_update_system())
        .add_system(tick::step_animation_system(0.0))
        .add_system(render::z_y_axis_player_system())
        .add_thread_local(tick::step_physics_system())
        .add_thread_local(tick::integrate_physics_system())
        .flush()
        .add_thread_local(tick::move_player_system())
        .add_system(render::track_player_system())
        .add_thread_local(tick::animate_player_system())
        .build();

    let mut draw_schedule = Schedule::builder()
        .add_thread_local(render::camera_system())
        .add_thread_local(render::clear_screen_system())
        .add_thread_local(render::render_system())
        .flush()
        .add_thread_local(render::camera_ui_system())
        .add_thread_local(tick::debug_input_system(false))
        .add_thread_local(render::draw_fps_system())
        .build();

    load(&mut world, &mut resources).await;
    physics_load(&mut world, &mut resources);
    'running: loop {
        let dt = Time {
            delta: get_frame_time(),
        };
        resources.insert(dt);

        if is_key_down(KeyCode::Escape) {
            #[cfg(debug_assertions)]
            break 'running;
        }

        step_schedule.execute(&mut world, &mut resources);
        draw_schedule.execute(&mut world, &mut resources);

        next_frame().await
    }

    Ok(())
}
