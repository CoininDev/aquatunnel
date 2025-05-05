mod comps;
mod entitites;
mod game;
mod input;
mod load;
mod physics;
mod resources;
mod sys;

#[macroquad::main("Aquatunnel")]
async fn main() {
    if let Err(err) = game::run_game().await {
        println!("{}", err);
    }
}
