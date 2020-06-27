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

                play_tic_tac_toe()

            }else if temp.trim() == "2" {

                play_twenty_one()

            }

        }
        Err(_) => (),
    }

    fn play_tic_tac_toe() {

        println!("Welcome to Tic-Tac-Toe!");

        let mut game = Game::new();

        game.play_game();

    }

    fn play_twenty_one() {

        println!("Pardon our dust!");

        let mut twentyone = Twentyone::new();

        twentyone.play_game()

    }









}
