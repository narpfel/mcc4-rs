use std::cell::RefCell;
#[cfg(feature = "noparallel")]
use std::iter::repeat;
use std::marker::PhantomData;

use rand::{Rng, XorShiftRng, weak_rng};

#[cfg(not(feature = "noparallel"))]
use rayon::prelude::*;
#[cfg(not(feature = "noparallel"))]
use rayon::iter::{repeatn};

use super::*;

pub const SIMULATIONS: usize = 100_000;


#[derive(Copy, Clone)]
pub struct AiPlayer<G: Game> {
    _game: PhantomData<G>,
}


impl<G: Game> AiPlayer<G> {
    pub fn new() -> AiPlayer<G> {
        AiPlayer {
            _game: PhantomData,
        }
    }
}

impl<G: Game> AiPlayer<G> {
    #[cfg(not(feature = "noparallel"))]
    fn simulate(&self, original_game: &G) -> Vec<(G::Move, i64)> {
        let me = original_game.current_player();

        original_game.valid_moves()
            .into_par_iter()
            .map_with(original_game.clone(), |ref mut initial_game, column| {
                initial_game.play(column).unwrap();
                let score = repeatn(initial_game.clone(), SIMULATIONS)
                    .map(|ref mut game| {
                        match simulate_game(game) {
                            Some(player) if player == me => 2,
                            Some(_) => -2,
                            _ => 1,
                        }
                    })
                    .sum();
                (column, score)
            })
            .collect()
    }

    #[cfg(feature = "noparallel")]
    fn simulate(&self, original_game: &G) -> Vec<(G::Move, i64)> {
        let me = original_game.current_player();

        original_game.valid_moves()
            .into_iter()
            .map(|column| {
                let mut initial_game = original_game.clone();
                initial_game.play(column).unwrap();
                let score = repeat(initial_game.clone()).take(SIMULATIONS)
                    .map(|ref mut game| {
                        match simulate_game(game) {
                            Some(player) if player == me => 2,
                            Some(_) => -2,
                            _ => 1,
                        }
                    })
                    .sum();
                (column, score)
            })
            .collect()
    }
}

impl<G: Game + 'static> PlayerTrait for AiPlayer<G> {
    type Game = G;

    fn make_move(&self, original_game: &G) -> G::Move {
        self.simulate(original_game).into_iter().max_by_key(|&(_, score)| score).unwrap().0
    }
}

pub fn simulate_game<G: Game>(game: &mut G) -> Option<Player> {
    thread_local!(static RNG: RefCell<XorShiftRng> = RefCell::new(weak_rng()));

    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();

        let mut valid_moves = vec![];
        loop {
            game.valid_moves_fast(&mut valid_moves);
            if valid_moves.is_empty() {
                return None;
            }
            else {
                if let Some(winner) = game.play(*rng.choose(&valid_moves).unwrap()).unwrap() {
                    return Some(winner);
                }
            }
        }
    })
}
