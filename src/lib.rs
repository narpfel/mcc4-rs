extern crate rand;

use std::io;
use std::io::prelude::*;

mod ai_player;
mod game;

pub use ai_player::AiPlayer;
pub use game::{Game, State, ArrayState, InvalidMove, Player};


pub trait PlayerTrait {
    fn make_move(&mut self, game: &Game) -> usize;
}


#[derive(Clone, Copy)]
pub struct HumanPlayer;

impl PlayerTrait for HumanPlayer {
    fn make_move(&mut self, game: &Game) -> usize {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        loop {
            print!("Player {}, place a stone. Which column? ", game.current_player());
            io::stdout().flush().unwrap();

            let input = lines.next().expect("Input failed").expect("Input failed");
            match input.parse::<usize>() {
                Ok(position) => return position.wrapping_sub(1),
                Err(_) => {
                    println!("Please enter a number.");
                }
            }
        }
    }
}
