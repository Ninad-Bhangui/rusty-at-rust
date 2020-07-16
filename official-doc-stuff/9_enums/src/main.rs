fn main() {
    println!("Hello, world!");

    let skyrim = Game::rpg(String::from("Skyrim"));
    let battlefield = Game::fps(String::from("Battlefield"));
    let civilization = Game::turnbased(String::from("Battlefield"));
}

enum Game {
    rpg(String),
    fps(String),
    turnbased(String),
}

impl Game {
    fn describe(&self) {
        println!("Name :");
    }
}

fn game_length_predictor(game: Game) -> i32 {
    match game {
        rpg => 40,
        fps => 8,
        turnbased => 20,
    }
}
