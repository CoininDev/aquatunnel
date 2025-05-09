use macroquad::{conf::Conf, miniquad};

mod common;
mod comps;
mod entitites;
mod game;
mod load;
mod resources;
mod sys;

fn conf() -> Conf {
    Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Aquatunnel".into(),
            fullscreen: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    if let Err(err) = game::run_game().await {
        println!("{}", err);
    }
}
