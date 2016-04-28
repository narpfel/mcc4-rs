extern crate rand;

extern crate mcc4;

use rand::Rng;

use mcc4::{Game, State, AiPlayer, HumanPlayer, PlayerTrait};

fn main() {
    let mut game = Game::new(7, 6);
    let mut human_player = HumanPlayer;
    let mut ai_player = AiPlayer;
    let mut players: Vec<&mut PlayerTrait> = vec![&mut human_player, &mut ai_player];
    rand::thread_rng().shuffle(&mut players);

    println!("\x1B[2J\x1B[H");
    println!("{}", game.state());
    'outer: loop {
        for player in players.iter_mut() {
            let move_ = player.make_move(&mut game);
            if let Ok(state) = game.play(move_) {
                println!("Player {} has moved {}", game.other_player(), move_);
                println!("{}", state);
            }

            if let Some(winner) = game.winner() {
                println!("Player {} has won.", winner);
                break 'outer;
            }
        }
    }
}
