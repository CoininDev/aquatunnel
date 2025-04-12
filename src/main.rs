mod comps;
mod game;
mod sys;

fn main() {
    if let Err(err) = game::run_game() {
        println!("{}", err);
    }
}
