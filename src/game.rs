use std::{collections::HashMap, sync::Arc, time::Instant};

use legion::{Resources, Schedule, World, systems::CommandBuffer};
use macroquad::{input::{is_key_down, KeyCode}, texture::Texture2D, window::next_frame};

use crate::{
    entitites::populate,
    input::{InputContext, InputSetup},
    physics::PhysicsContext,
    sys::{
        load::{load_physics_system, load_spritesheet_system, load_system},
        render, tick,
    },
};

pub struct Time {
    pub last: Instant,
    pub delta: f32,
}

pub async fn run_game() -> Result<(), String> {
    let textures: HashMap<String, Arc<Texture2D>> = HashMap::new();
    let mut world = World::default();
    let mut resources = Resources::default();
    let time = Time {
        last: Instant::now(),
        delta: 0.0,
    };

    let input_ctx = InputContext::new(InputSetup::default());
    let physics_ctx = PhysicsContext::default();

    resources.insert(textures);
    resources.insert(input_ctx);
    resources.insert(physics_ctx);
    resources.insert(time);
    resources.insert(CommandBuffer::new(&world));

    populate(&mut world);

    let mut load_schedule = Schedule::builder()
        .add_thread_local(load_system())
        .add_thread_local(load_spritesheet_system())
        .add_system(load_physics_system())
        .build();

    let mut step_schedule = Schedule::builder()
        .add_system(tick::time_update_system())
        .add_thread_local(tick::input_update_system())
        .add_system(tick::step_animation_system(0.0))
        .add_system(tick::step_physics_system())
        .add_system(tick::physics_integration_system())
        .flush()
        .add_thread_local(tick::move_player_system())
        .build();

    let mut draw_schedule = Schedule::builder()
        .add_thread_local(render::clear_screen_system())
        .add_thread_local(render::render_system())
        .add_thread_local(render::draw_fps_system())
        .build();

    load_schedule.execute(&mut world, &mut resources);
    'running:  loop {
        if is_key_down(KeyCode::Escape) {
            break 'running;
        }

        step_schedule.execute(&mut world, &mut resources);
        draw_schedule.execute(&mut world, &mut resources);

        next_frame().await
    }

    Ok(())
}