use std::{collections::HashMap, sync::Arc};

use legion::{Entity, Resources, Schedule, World, systems::CommandBuffer};
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
    load::load,
    sys::{
        render::{self, camera_system, camera_ui_system},
        tick,
    },
};

pub struct Time {
    pub delta: f32,
}

pub struct Track {
    pub pos: Vec2,
}

pub async fn run_game() -> Result<(), String> {
    let textures: HashMap<String, Arc<Texture2D>> = HashMap::new();
    let mut world = World::default();
    let mut resources = Resources::default();
    let camera = Box::new(Camera2D::default());
    let input_ctx = InputContext::new(InputSetup::default());
    let track: Track = Track { pos: Vec2::ZERO };
    resources.insert(track);
    resources.insert(textures);
    resources.insert(input_ctx);
    resources.insert(camera);
    resources.insert(CommandBuffer::new(&world));

    populate(&mut world);

    // Systems involving macroquad rendering or input requires local thread
    let mut step_schedule = Schedule::builder()
        .add_thread_local(tick::input_update_system())
        .add_system(tick::step_animation_system(0.0))
        .add_system(render::z_y_axis_player_system())
        .flush()
        .add_system(tick::move_player_system())
        .add_system(render::track_player_system())
        .add_thread_local(tick::animate_player_system())
        .build();

    let mut draw_schedule = Schedule::builder()
        .add_thread_local(camera_system())
        .add_thread_local(render::clear_screen_system())
        .add_thread_local(render::render_system())
        .flush()
        .add_thread_local(camera_ui_system())
        .add_thread_local(render::draw_fps_system())
        .build();

    load(&mut world, &mut resources).await;
    'running: loop {
        let dt = Time {
            delta: get_frame_time(),
        };
        resources.insert(dt);

        if is_key_down(KeyCode::Escape) {
            break 'running;
        }

        step_schedule.execute(&mut world, &mut resources);
        draw_schedule.execute(&mut world, &mut resources);

        next_frame().await
    }

    Ok(())
}
