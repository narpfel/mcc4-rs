use std::marker::PhantomData;
use std::thread;
use std::sync::mpsc;

use rand::{StdRng, Rng, SeedableRng};

use super::*;

pub const SIMULATIONS: usize = 100_000;


#[derive(Clone, Copy)]
pub struct AiPlayer<'a, G: Game> {
    seed: Option<&'a [usize]>,
    _game: PhantomData<G>,
}


impl<'a, G: Game> AiPlayer<'a, G> {
    pub fn new() -> AiPlayer<'a, G> {
        AiPlayer {
            seed: None,
            _game: Default::default(),
        }
    }

    pub fn with_seed(seed: &'a [usize]) -> AiPlayer<'a, G> {
        AiPlayer {
            seed: Some(seed),
            _game: Default::default(),
        }
    }

    fn new_rng(&self) -> StdRng {
        match self.seed {
            Some(seed) => StdRng::from_seed(seed),
            None => StdRng::new().expect(
                "Could not create random number generator, not enough entropy"
            ),
        }
    }
}


impl<'a, G: Game + 'static> PlayerTrait for AiPlayer<'a, G> {
    type Game = G;

    fn make_move(&self, original_game: &G) -> G::Move {
        let me = original_game.current_player();
        let valid_moves = original_game.valid_moves();

        let (tx, rx) = mpsc::channel();
        for column in &valid_moves {
            let (mut initial_game, tx) = (original_game.clone(), tx.clone());
            let column = *column;
            let mut rng = self.new_rng();

            initial_game.play(column).unwrap();
            thread::spawn(move || {
                let mut score = 0;
                for _ in 0..SIMULATIONS {
                    let game = initial_game.clone();
                    score += match simulate_game(&mut rng, game) {
                        Some(player) => if player == me { 2 } else { -2 },
                        _ => 1
                    };
                }
                tx.send((column, score)).unwrap();
            });
        }

        let mut scores = Vec::with_capacity(valid_moves.len());
        for _ in &valid_moves {
            let (column, score) = rx.recv().unwrap();
            scores.push((column, score));
        }

        scores.iter().max_by_key(|&&(_, score)| score).unwrap().0
    }
}


pub fn simulate_game<R: Rng, G: Game>(rng: &mut R, mut game: G) -> Option<Player> {
    loop {
        let valid_moves = game.valid_moves();
        if valid_moves.is_empty() {
            return None;
        }
        if let Some(winner) = game.play(*rng.choose(&valid_moves).unwrap()).unwrap() {
            return Some(winner);
        }
    }
}
