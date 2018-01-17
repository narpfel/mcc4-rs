extern crate mcc4;

use std::time::{Instant, Duration};

use mcc4::*;
use mcc4::ai_player::SIMULATIONS;


pub const NANOS_PER_SECOND: u64 = 1_000_000_000;


fn as_fractional_secs(duration: Duration) -> f64 {
    (duration.as_secs() * NANOS_PER_SECOND + duration.subsec_nanos() as u64) as f64
        / NANOS_PER_SECOND as f64
}


fn main() {
    let columns = 7;
    let game = ConnectFour::<BitState>::new(columns, 6).unwrap();
    let benchmark_player = AiPlayer::new();
    let now = Instant::now();
    benchmark_player.make_move(&game);
    let seconds = as_fractional_secs(now.elapsed());
    println!("{} seconds elapsed", seconds);
    println!("{:?} games per second", (columns * SIMULATIONS) as f64 / seconds);
}
