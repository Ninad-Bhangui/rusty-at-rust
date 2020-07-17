fn main() {
    println!("Hello, world!");

    let skyrim = Game::Rpg(String::from("Skyrim"));
    let battlefield = Game::Fps(String::from("Battlefield"));
    let civilization = Game::Turnbased(String::from("Sid Meir's Civilization 5"));

    println!("{}", skyrim.describe());
    println!(
        "Length of {:?}! is predicted to be {}",
        skyrim,
        game_length_predictor(&skyrim)
    );
    println!("{}", battlefield.describe());
    println!(
        "Length of {:?}! is predicted to be {}",
        battlefield,
        game_length_predictor(&battlefield)
    );
    println!("{}", civilization.describe());
    println!(
        "Length of {:?}! is predicted to be {}",
        civilization,
        game_length_predictor(&civilization)
    );
}
#[derive(Debug)]
enum Game {
    Rpg(String),
    Fps(String),
    Turnbased(String),
}

impl Game {
    fn describe(&self) -> String {
        match self {
            Game::Rpg(val) => format!("{} is a role playing game", val),
            Game::Fps(val) => format!("{} is a first person shooter", val),
            Game::Turnbased(val) => format!("{} is a turn based strategy game", val),
        }
    }
}

fn game_length_predictor(game: &Game) -> i32 {
    match game {
        Game::Rpg(_val) => 40,
        Game::Fps(_val) => 8,
        Game::Turnbased(_val) => 20,
    }
}
