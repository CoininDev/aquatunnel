use std::collections::HashMap;

use fastnoise_lite::{FastNoiseLite, NoiseType};
use legion::{Resources, World};
use macroquad::{
    camera::Camera2D,
    input::{KeyCode, is_key_down, is_key_pressed},
    math::{UVec2, Vec2},
    miniquad::date::now,
    time::get_frame_time,
    window::next_frame,
};

use crate::{
    entities::populate,
    load::{load, physics_load},
    resources::{
        chunk_manager::ChunkManager,
        input::{InputContext, InputSetup},
        physics, *,
    },
    sys,
};

pub async fn run_game() -> Result<(), String> {
    let mut loaded_textures = HashMap::new();

    loop {
        let mut world = World::default();
        let mut resources = Resources::default();

        //Registering physics resources
        physics::init_physics(&mut resources);
        resources.insert(Track { pos: Vec2::ZERO });
        resources.insert(Textures(loaded_textures));
        resources.insert(InputContext::new(InputSetup::default()));
        resources.insert(crate::resources::GuiCommandBuffer::default());

        let mut noise = FastNoiseLite::new();
        noise.set_seed(Some(now().floor() as i32));
        noise.set_noise_type(Some(NoiseType::Perlin));

        resources.insert(ChunkManager::new(
            noise,
            Vec2::ONE * 40.0,
            0.01,
            UVec2::ONE * 16,
            Vec2::ONE * 0.16,
            9,
            12,
        ));
        resources.insert(Box::new(Camera2D::default()));

        populate(&mut world);
        let (mut step_schedule, mut draw_schedule) = sys::populate();

        // Systems involving macroquad rendering or input requires local thread
        load(&mut world, &mut resources).await;
        physics_load(&mut world, &mut resources);

        let mut should_restart = false;

        'running: loop {
            let dt = Time {
                delta: get_frame_time(),
            };
            resources.insert(dt);

            if is_key_pressed(KeyCode::R) {
                should_restart = true;
                next_frame().await;
                break 'running;
            }

            if is_key_down(KeyCode::Escape) {
                #[cfg(debug_assertions)]
                break 'running;
            }

            step_schedule.execute(&mut world, &mut resources);
            draw_schedule.execute(&mut world, &mut resources);
            next_frame().await
        }

        loaded_textures = resources.remove::<Textures>().unwrap().0;

        if !should_restart {
            break;
        }
    }

    Ok(())
}
