use std::collections::HashMap;

use fastnoise_lite::{FastNoiseLite, NoiseType};
use legion::{Resources, World};
use macroquad::{
    camera::Camera2D,
    input::{KeyCode, is_key_down},
    math::{UVec2, Vec2},
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    entitites::populate,
    load::{load, physics_load},
    resources::{
        chunk_manager::ChunkManager,
        input::{InputContext, InputSetup},
        physics, *,
    },
    sys,
};

pub async fn run_game() -> Result<(), String> {
    let mut world = World::default();
    let mut resources = Resources::default();

    //Registering physics resources
    physics::init_physics(&mut resources);

    resources.insert(Track { pos: Vec2::ZERO });
    resources.insert(Textures(HashMap::new()));
    resources.insert(InputContext::new(InputSetup::default()));

    let mut noise = FastNoiseLite::new();
    noise.set_seed(None);
    noise.set_noise_type(Some(NoiseType::Perlin));

    resources.insert(ChunkManager::new(
        noise,
        Vec2::ONE * 40.,
        0.01,
        UVec2::ONE * 16,
        Vec2::ONE * 0.16,
        1,
        12,
    ));
    resources.insert(Box::new(Camera2D::default()));
    resources.insert(RenderQueue(Vec::new()));

    populate(&mut world);
    let (mut step_schedule, mut draw_schedule) = sys::populate();


    // Systems involving macroquad rendering or input requires local thread
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
