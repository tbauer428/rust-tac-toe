mod twentyone;
mod game;

extern crate rand;

use crate::game::Game;
use crate::twentyone::Twentyone;
use std::io;


fn main() {

    let mut player_input = String::new();

    println!("Would you like to play Tic-Tac-Toe? Or 21?:  (1/2)");

    match io::stdin().read_line(&mut player_input) {
        Ok(_) => {

            let temp = player_input.to_lowercase();

            if temp.trim() == "1" {

                println!("Welcome to Tic-Tac-Toe!");

                let mut game = Game::new();

                game.play_game();

            }else if temp.trim() == "2" {

                println!("Pardon our dust!");

                let mut twentyone = Twentyone::new();

                twentyone.play_game()

            }

        }
        Err(_) => (),
    }









}
