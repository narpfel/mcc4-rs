extern crate mcc4;

use std::time::Instant;

use mcc4::*;
use mcc4::ai_player::SIMULATIONS;


fn main() {
    let columns = 7;
    let game = ConnectFour::<BitState>::new(columns, 6).unwrap();
    let benchmark_player = AiPlayer::new();
    let now = Instant::now();
    benchmark_player.make_move(&game);
    let time = now.elapsed();
    let seconds = (time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64) as f64 / 1_000_000_000.;
    println!("{} seconds elapsed", seconds);
    println!("{:?} games per second", (columns * SIMULATIONS) as f64 / seconds);
}
