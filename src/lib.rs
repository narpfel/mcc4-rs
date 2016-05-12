extern crate rand;

use std::io;
use std::io::prelude::*;
use std::marker::PhantomData;

pub mod ai_player;
pub mod connect_four;
mod game;

pub use ai_player::AiPlayer;
pub use game::{Game, Player};
pub use connect_four::*;


pub trait PlayerTrait {
    type Game: Game;

    fn make_move(&self, game: &Self::Game) -> <Self::Game as Game>::Move;

    fn invalid_move(&self, _move: <Self::Game as Game>::InvalidMove) {
        // ignore by default
    }
}


#[derive(Clone, Copy)]
pub struct HumanPlayer<G: Game> {
    _game: PhantomData<G>,
}

impl<G: Game> HumanPlayer<G> {
    pub fn new() -> HumanPlayer<G> {
        HumanPlayer {
            _game: Default::default(),
        }
    }
}

impl<G: Game> PlayerTrait for HumanPlayer<G> {
    type Game = G;

    fn make_move(&self, game: &G) -> G::Move {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        loop {
            print!("Player {}, place a stone. Which column? ", game.current_player());
            io::stdout().flush().unwrap();

            let input = lines.next().expect("Input failed").expect("Input failed");
            match input.parse::<G::Move>() {
                Ok(position) => return position,
                Err(_) => {
                    println!("Please enter a number.");
                }
            }
        }
    }

    fn invalid_move(&self, move_: <Self::Game as Game>::InvalidMove) {
        println!("Invalid input: {:?}", move_)
    }
}


#[derive(Copy, Clone)]
pub struct AiAidedPlayer<G: Game> {
    _game: PhantomData<G>,
}

impl<G: Game> AiAidedPlayer<G> {
    pub fn new() -> AiAidedPlayer<G> {
        AiAidedPlayer {
            _game: Default::default(),
        }
    }
}

impl<G: Game + 'static> PlayerTrait for AiAidedPlayer<G> {
    type Game = G;

    fn make_move(&self, game: &G) -> G::Move {
        println!("The AI would choose column {}.", AiPlayer::new().make_move(game));
        HumanPlayer::new().make_move(game)
    }
}
