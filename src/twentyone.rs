

enum Turn {
    Player,
    Bot,
}

enum Action {
    Hit,
    Stay,
}

pub struct Twentyone {
    current_turn: Turn,
}

// Pick a number for the dealer, pick a number for the player
// If the dealer has 21 and the player does not -> dealer win



impl Twentyone {
    pub fn new() -> Twentyone {
        Twentyone {
            current_turn: Turn::Player,
        }
    }

    pub fn play_game(&mut self) {


    }





}


