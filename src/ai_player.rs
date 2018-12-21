use std::iter::repeat;

use super::*;

use crate::connect_four::Move;

pub const SIMULATIONS: usize = 1_000_000;


#[derive(Copy, Clone)]
pub struct MonteCarloPlayer;

impl MonteCarloPlayer {
    fn simulate(&self, original_game: &ConnectFour) -> Vec<(Move, i64)> {
        let me = original_game.current_player;

        original_game.state.valid_moves()
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

    pub fn make_move(&self, original_game: &ConnectFour) -> Move {
        self.simulate(original_game).into_iter().max_by_key(|&(_, score)| score).unwrap().0
    }
}

pub fn simulate_game(game: &mut ConnectFour) -> Option<Player> {
    let rng = &mut XorShiftRng::from_seed();

    let mut valid_moves = vec![];
    loop {
        game.state.valid_moves_fast(&mut valid_moves);
        if valid_moves.is_empty() {
            return None;
        }
        else {
            if let Some(winner) = game.play(*choose(&mut *rng, &valid_moves)).unwrap() {
                return Some(winner);
            }
        }
    }
}

fn choose<'a, T>(rng: &mut XorShiftRng, ts: &'a [T]) -> &'a T {
    &ts[rand_in_range(ts.len(), rng)]
}

/// Generate a random `r: usize` that satisfies `0 <= r < upper`.
///
/// The algorithm implemented in this function performs significantly better than the standard
/// modulo reduction algorithm, because it avoids the costly division operation.
/// It is not unbiased, but its bias is insignificant.
///
/// This algorithm was adapted from a C implementation given by Daniel Lemire in his blog:
/// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
fn rand_in_range(upper: usize, rng: &mut XorShiftRng) -> usize {
    let random = rng.next_u32() as usize;
    (upper * random) >> 32
}

use core::num::Wrapping as w;

#[derive(Clone)]
pub struct XorShiftRng {
    x: w<u32>,
    y: w<u32>,
    z: w<u32>,
    w: w<u32>,
}

impl XorShiftRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }

    fn from_seed() -> Self {
        let seed_u32 = [0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED];

        XorShiftRng {
            x: w(seed_u32[0]),
            y: w(seed_u32[1]),
            z: w(seed_u32[2]),
            w: w(seed_u32[3]),
        }
    }
}
