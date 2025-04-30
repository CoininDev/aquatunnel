mod comps;
mod entitites;
mod game;
mod input;
mod load;
mod sys;

#[macroquad::main("Lo")]
/// ```
async fn main() {
    if let Err(err) = game::run_game().await {
        println!("{}", err);
    }
}
