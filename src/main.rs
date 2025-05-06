mod comps;
mod entitites;
mod game;
mod load;
mod resources;
mod sys;

#[macroquad::main("Aquatunnel")]
async fn main() {
    if let Err(err) = game::run_game().await {
        println!("{}", err);
    }
}
