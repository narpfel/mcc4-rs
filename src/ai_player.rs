use std::cell::RefCell;
#[cfg(feature = "noparallel")]
use std::iter::repeat;
use std::marker::PhantomData;

use rand::{Rng, RngCore, SeedableRng};
use rand_os::OsRng;
use rand_xoshiro::Xoshiro256StarStar;

#[cfg(not(feature = "noparallel"))]
use rayon::prelude::*;
#[cfg(not(feature = "noparallel"))]
use rayon::iter::{repeatn};

use super::*;

pub const SIMULATIONS: usize = 100_000;


#[derive(Copy, Clone)]
pub struct MonteCarloPlayer<G: Game> {
    _game: PhantomData<G>,
}

impl<G: Game> Default for MonteCarloPlayer<G> {
    fn default() -> MonteCarloPlayer<G> {
        MonteCarloPlayer {
            _game: PhantomData,
        }
    }
}

impl<G: Game> MonteCarloPlayer<G> {
    pub fn new() -> MonteCarloPlayer<G> {
        Self::default()
    }

    #[cfg(not(feature = "noparallel"))]
    fn simulate(&self, original_game: &G) -> Vec<(G::Move, i64)> {
        let me = original_game.current_player();

        original_game.valid_moves()
            .into_par_iter()
            .map_with(original_game.clone(), |ref mut initial_game, column| {
                initial_game.play(column).unwrap();
                let score = repeatn(initial_game.clone(), SIMULATIONS)
                    .map(|game| {
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
                    .map(|game| {
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

impl<G: Game + 'static> PlayerTrait for MonteCarloPlayer<G> {
    type Game = G;

    fn make_move(&self, original_game: &G) -> G::Move {
        self.simulate(original_game).into_iter().max_by_key(|&(_, score)| score).unwrap().0
    }
}

pub fn simulate_game(game: impl Game) -> Option<Player> {
    thread_local!(static RNG: RefCell<Xoshiro256StarStar> = RefCell::new(new_rng()));

    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();

        random_playout(&mut *rng, game)
    })
}

fn random_playout(rng: &mut impl Rng, mut game: impl Game) -> Option<Player> {
    let mut valid_moves = vec![];
    loop {
        game.valid_moves_fast(&mut valid_moves);
        if valid_moves.is_empty() {
            return None;
        }
        else {
            if let Some(winner) = game.play(*choose(rng, &valid_moves)).unwrap() {
                return Some(winner);
            }
        }
    }
}

fn choose<'a, R: Rng, T>(rng: &mut R, ts: &'a [T]) -> &'a T {
    &ts[rand_in_range(ts.len() as u32, rng) as usize]
}

/// Generate a random `r: usize` that satisfies `0 <= r < upper`.
///
/// The algorithm implemented in this function performs significantly better than the standard
/// modulo reduction algorithm, because it avoids the costly division operation.
/// It is not unbiased, but its bias is insignificant.
///
/// This algorithm was adapted from a C implementation given by Daniel Lemire in his blog:
/// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
fn rand_in_range<R: Rng>(upper: u32, rng: &mut R) -> u32 {
    let random = rng.next_u32() as u64;
    ((upper as u64 * random) >> 32) as u32
}

fn new_rng() -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(
        OsRng::new()
        .expect("could not gather entropy from the system")
        .next_u64()
    )
}
