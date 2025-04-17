mod comps;
mod entitites;
mod game;
mod input;
mod render;
mod sys;

fn main() {
    if let Err(err) = game::run_game() {
        println!("{}", err);
    }
}
