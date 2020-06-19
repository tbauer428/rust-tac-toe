extern crate rand;

use crate::game::Game;

mod game;

fn main() {
    println!("Welcome to Tic-Tac-Toe!");

    let mut game = Game::new();

    game.play_game();
}
