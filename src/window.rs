use sdl2::{
    EventPump,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

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
        .present_vsync()
        //.accelerated()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    Ok((canvas, texture_creator, event_pump))
}
