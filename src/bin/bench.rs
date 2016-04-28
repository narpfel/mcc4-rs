extern crate rand;

extern crate mcc4;


use std::collections::HashMap;
use std::thread;
use std::time::Instant;
use std::sync::mpsc;

use rand::{StdRng, SeedableRng};

use mcc4::*;
use mcc4::ai_player::{find_valid_moves, simulate_game};


const SIMULATIONS: usize = 100_000;


#[derive(Clone, Copy)]
pub struct BenchmarkPlayer;


impl BenchmarkPlayer {
    fn benchmark(&self, original_game: &Game) -> usize {
        let me = original_game.current_player();
        let valid_moves = find_valid_moves(original_game);
        let initial_games: Vec<_> = valid_moves.iter().map(|column| {
            let mut game = original_game.clone();
            game.play(*column).unwrap();
            game
        }).collect();

        let (tx, rx) = mpsc::channel();
        for (column, initial_game) in valid_moves.iter().zip(&initial_games) {
            let (column, initial_game, tx) = (column.clone(), initial_game.clone(), tx.clone());
            thread::spawn(move || {
                let mut score = 0;
                let mut rng = StdRng::from_seed(&[1, 2, 3, 42]);
                for _ in 0..SIMULATIONS {
                    let mut game = initial_game.clone();
                    score += match simulate_game(&mut rng, &mut game) {
                        Some(player) => if player == me { 2 } else { -2 },
                        _ => 1
                    };
                }
                tx.send((column, score)).unwrap();
            });
        }

        let mut scores = HashMap::new();
        for _ in 0..valid_moves.len() {
            let (column, score) = rx.recv().unwrap();
            scores.insert(column, score);
        }

        *scores.iter().max_by_key(|&(_, score)| *score).unwrap().0
    }
}


fn main() {
    let game = Game::new(7, 6);
    let benchmark_player = BenchmarkPlayer;
    let now = Instant::now();
    benchmark_player.benchmark(&game);
    let time = now.elapsed();
    println!("{:?}", time);
    let seconds = (time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64) as f64 / 1_000_000_000.;
    println!("{:?} games per second", 700_000. / seconds);
}
