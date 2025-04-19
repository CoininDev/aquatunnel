use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc, time::Instant};

use legion::{Resources, Schedule, World, systems::CommandBuffer};
use sdl2::{EventPump, event::Event, keyboard::Keycode, render::Texture};

use crate::{
    entitites::populate,
    input::{InputContext, InputSetup},
    physics::PhysicsContext,
    window::init_sdl,
    sys::{
        load::{load_physics_system, load_spritesheet_system, load_system},
        render, tick,
    },
};

pub struct Time {
    pub last: Instant,
    pub delta: f32,
}

pub fn run_game() -> Result<(), String> {
    let (canvas, texture_creator, event_pump) = init_sdl()?;
    let event_pump = Rc::new(RefCell::new(event_pump));
    let textures: HashMap<String, Arc<Texture<'_>>> = HashMap::new();
    let mut world = World::default();
    let mut resources = Resources::default();

    let ttf_ctx = Arc::new(sdl2::ttf::init().unwrap());
    let time = Time {
        last: Instant::now(),
        delta: 0.0,
    };

    let input_ctx = InputContext::new(event_pump.clone(), InputSetup::default());
    let physics_ctx = PhysicsContext::default();

    resources.insert(textures);
    resources.insert(canvas);
    resources.insert(texture_creator);
    resources.insert(input_ctx);
    resources.insert(physics_ctx);
    resources.insert(ttf_ctx);
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
        .add_thread_local(render::draw_fps_system(
            0,
            String::from("0"),
            Instant::now(),
        ))
        .add_thread_local(render::present_system())
        .build();

    load_schedule.execute(&mut world, &mut resources);
    'running: loop {
        if check_exit(event_pump.clone()) {
            break 'running;
        }
        step_schedule.execute(&mut world, &mut resources);
        draw_schedule.execute(&mut world, &mut resources);
    }

    Ok(())
}

fn check_exit(event_pump: Rc<RefCell<EventPump>>) -> bool {
    for event in event_pump.borrow_mut().poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::ESCAPE),
                ..
            } => return true,

            _ => {}
        }
    }

    return false;
}
