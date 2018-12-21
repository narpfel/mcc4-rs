use mcc4::*;


/// This example failed in the first version of `BitBoard` because of a typo in
/// `BitBoard::has_winner()`.
#[test]
fn test_bitboard_has_won_with_moves_332145223344455() {
    let mut game = ConnectFour::<BitState>::new(7, 6).unwrap();
    let moves = [3, 3, 2, 1, 4, 5, 2, 2, 3, 3, 4, 4, 4, 5, 5];
    for move_ in &moves {
        let winner = game.play(*move_).unwrap();
        assert!(winner.is_none());
    }
}
