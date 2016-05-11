extern crate rand;

extern crate mcc4;

use rand::Rng;

use mcc4::*;

fn main() {
    let mut game = ConnectFour::<VecState>::new(7, 6);
    let human_player = HumanPlayer::new();
    let ai_player = AiPlayer::new();
    let mut players: Vec<&PlayerTrait<Game=_>> = vec![&human_player, &ai_player];
    rand::thread_rng().shuffle(&mut players);

    println!("\x1B[2J\x1B[H");
    println!("{}", game.state());
    'outer: loop {
        for player in &players {
            loop {
                let move_ = player.make_move(&mut game);
                match game.play(move_) {
                    Ok(_) => {
                        print!("\x1B[2J\x1B[H");
                        println!("Player {} has moved {}", game.other_player(), move_);
                        println!("{}", game.state());
                        break;
                    },
                    Err(err) => println!("Invalid input: {:?}", err),
                }
            }

            if let Some(winner) = game.winner() {
                println!("Player {} has won.", winner);
                break 'outer;
            }

            if game.valid_moves().is_empty() {
                println!("Draw.");
                break 'outer;
            }
        }
    }
}
