use std::{collections::HashMap, sync::Arc, time::Instant};

use legion::{Resources, Schedule, World, systems::CommandBuffer};
use sdl2::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::FPoint,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};

use crate::{
    comps::{DebugSprite, Transform},
    sys::{load::load_system, render, tick},
};

pub struct Time {
    pub last: Instant,
    pub delta: f32,
}

pub fn init_sdl<'a>() -> Result<(WindowCanvas, TextureCreator<WindowContext>, EventPump), String> {
    let sdl_ctx = sdl2::init()?;

    let video = sdl_ctx.video()?;
    let event_pump = sdl_ctx.event_pump()?;

    let window = video
        .window("Aquatunnel", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window
        .clone()
        .into_canvas()
        //.present_vsync()
        .accelerated()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    Ok((canvas, texture_creator, event_pump))
}

pub fn populate(world: &mut World) {
    world.push((
        Transform::default(),
        DebugSprite {
            size: FPoint::new(40.0, 40.0),
            color: Color::CYAN,
        },
    ));
}

pub fn run_game() -> Result<(), String> {
    let (canvas, texture_creator, input_ctx) = init_sdl()?;
    let textures: HashMap<String, Arc<Texture<'_>>> = HashMap::new();
    let mut world = World::default();
    let mut resources = Resources::default();

    let ttf_ctx = Arc::new(sdl2::ttf::init().unwrap());
    let time = Time {
        last: Instant::now(),
        delta: 0.0,
    };
    // let mut fps = String::from("0");

    resources.insert(textures);
    resources.insert(canvas);
    resources.insert(texture_creator);
    resources.insert(input_ctx);
    resources.insert(ttf_ctx);
    resources.insert(time);
    resources.insert(CommandBuffer::new(&world));

    populate(&mut world);

    let mut load_schedule = Schedule::builder().add_thread_local(load_system()).build();
    let mut step_schedule = Schedule::builder()
        .add_system(tick::delta_update_system())
        .add_system(tick::spawn_system(0))
        .add_system(tick::move_squares_system())
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
        if check_exit(&mut *resources.get_mut::<EventPump>().unwrap()) {
            break 'running;
        }
        step_schedule.execute(&mut world, &mut resources);
        draw_schedule.execute(&mut world, &mut resources);
    }

    Ok(())
}

fn check_exit(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
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
