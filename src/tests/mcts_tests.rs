use crate::board::Board;
use crate::mcts;
use crate::tests::do_moves_and_check_validity;
use board_game_traits::board::Board as BoardTrait;
use pgn_traits::pgn::PgnBoard;

#[test]
fn win_in_two_moves_test() {
    let move_strings = ["c3", "e5", "c2", "d5", "c1", "c5", "d3", "a4", "e3"];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["b4", "b5", "Cb4", "Cb5"]),
    );
}

#[test]
fn black_win_in_one_move_test() {
    let move_strings = [
        "c2", "b4", "d2", "c4", "b2", "c3", "d3", "b3", "c2+", "b3>", "d3<", "c4-", "d4",
        "4c3<22", "c2", "c4", "d4<", "b4>", "d3", "b4", "b1", "d4", "b2+", "2a3>", "e1",
        "5b3-23", "b3", "d1", "e1<", "a5", "e1", "b5", "b3+", "2c4<", "e1+",
    ];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["3b4-", "b3", "Cb3", "e4", "Ce4", "c3<"]),
    );
}

#[test]
fn white_can_win_in_one_move_test() {
    let move_strings = ["c2", "b4", "d2", "c4", "b2", "d4", "e2", "c3"];

    plays_correct_move_property(&move_strings, TacticAnswer::PlayMoves(&["a2", "Ca2"]));
}

#[test]
fn black_avoid_loss_in_one_test() {
    let move_strings = ["c2", "b4", "d2", "c4", "b2", "d4", "e2"];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["a2", "Ca2", "Sa2"]),
    );
}

#[test]
fn black_avoid_loss_in_one_test2() {
    let move_strings = [
        "c2", "b4", "d2", "d4", "b2", "c4", "e2", "a2", "c3", "b3", "b2+", "c4-", "c2+", "b2",
        "b1", "d1", "c2", "a3", "2b3-", "a2>", "b1+",
    ];
    plays_correct_move_property(&move_strings, TacticAnswer::PlayMoves(&["d1+"]));
}

#[test]
fn black_avoid_loss_in_one_test3() {
    let move_strings = [
        "c2", "c3", "d2", "d3", "d2+", "c4", "d2", "b4", "c2+", "c4-", "2d3<", "d4", "b2", "a5",
        "c2", "a2", "b1", "a2>", "b1+", "d4-", "5c3>23",
    ];

    plays_correct_move_property(&move_strings, TacticAnswer::PlayMoves(&["Ca2", "Sa2"]));
}

#[test]
fn black_avoid_less_in_one_test5() {
    let move_strings = [
        "c2", "b3", "d2", "c3", "b2", "d4", "b2+", "d3", "d2+", "c3>", "Cc3", "b4", "c3>",
        "d2", "2d3-", "b2", "c2<", "b4-", "2b2+", "c2", "3d2<", "d1", "b2", "c4", "2d3+", "c4>",
        "e1", "c4", "b4", "3d4<12", "d2", "d1+", "4c2>", "3b4-", "b2+", "d4-", "3b3-12",
    ];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["Sb4", "Cb4", "Sb5", "Cb5", "c4<", "2c4<"]),
    );
}

#[test]
fn white_avoid_loss_in_one_test() {
    let move_strings = [
        "c3", "c4", "b4", "c4-", "d2", "b5", "b3", "b5-", "b3>", "d4", "2c3-", "c4", "d3",
        "d4-", "d4", "c4-", "b2", "c4", "d4-", "2c3>", "d2+", "Sb3", "5d3-23", "b3-", "d4",
        "2b2>11", "3c2+21", "b3", "b2", "b3-", "c2", "b3", "c5", "2b2>", "b2", "b3-", "b3",
        "2b4-", "d5", "b4", "2c4<", "3b3+", "2c3-", "2b2>", "3d1<", "3c2-", "d1", "5b4-14",
    ];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["Cb5", "Sb5", "b5", "c5<", "d1<"]),
    );
}

#[test]
fn white_avoid_loss_in_one_test2() {
    let move_strings = [
        "c3", "c4", "b3", "b4", "d3", "b4-", "b2", "b4", "d4", "c4-", "d2", "c4", "c2", "2b3-",
        "c2+", "c4-", "d3<", "d5", "4c3>22", "d5-", "2d3+", "Sc4", "3d4-", "c4-", "c4",
        "2c3>", "c4<", "5d3+", "2b4-11", "b3-", "Se1", "4d4<13", "c3", "5b2>32", "c3-", "3d2<",
        "b3", "3b4-", "b4", "c4<", "d3", "c4", "d3+", "c4>", "e4", "2d4>", "2e3+", "2d4>", "d3",
        "a4",
    ];

    plays_correct_move_property(
        &move_strings,
        TacticAnswer::PlayMoves(&["Cd4", "Sd4", "Cc4", "Sc4"]),
    );
}

#[test]
fn do_not_suicide_as_black_test() {
    let move_strings = [
        "c2", "c4", "d2", "c3", "b2", "d3", "d2+", "b3", "d2", "b4", "c2+", "b3>", "2d3<",
        "c4-", "d4", "5c3<23", "c2", "c4", "d4<", "d3", "d2+", "c3+", "Cc3", "2c4>", "c3<",
        "d2", "c3", "d2+", "c3+", "b4>", "2b3>11", "3c4-12", "d2", "c4", "b4", "c5", "b3>",
        "c4<", "3c3-", "e5", "e2",
    ];

    let mut board = Board::default();
    do_moves_and_check_validity(&mut board, &move_strings);

    let mut moves = vec![];
    board.generate_moves(&mut moves);
    assert!(!moves.contains(&board.move_from_san("2a3-11").unwrap()));
}

#[test]
fn do_not_suicide_as_black_test2() {
    let move_strings = [
        "c2", "d3", "d2", "c3", "d2+", "c4", "d2", "b3", "c2+", "b3>", "2d3<", "c4-", "b2",
        "5c3+23", "c2", "d3", "d2+", "b4", "d2", "d4", "2d3+", "b3", "b2+", "b4-", "d3", "b2",
        "b1", "b2>", "d2<", "c3-", "c1", "3b3>12", "c1+", "c3-", "3d4-", "4c2<22", "b1+",
        "2a2>", "d2", "3b2<", "d2<", "2b2>", "d4", "d2", "5d3-14",
    ];

    let mut board = Board::default();
    do_moves_and_check_validity(&mut board, &move_strings);

    let mut moves = vec![];
    board.generate_moves(&mut moves);
    assert!(!moves.contains(&board.move_from_san("2c5>11").unwrap()));
}

#[test]
fn do_not_suicide_as_black_test3() {
    let move_strings = [
        "c2", "c3", "d2", "b4", "c2+", "d4", "c2", "b2", "c2<", "d5", "c2", "a3", "e2", "a2",
        "b1", "a1", "d3", "c4", "2c3+", "d4<", "Cc3", "3c4>12", "c3+", "e1", "c3", "a1>", "d1",
        "c5", "b5", "b4+", "d3+",
    ];

    let mut board = Board::default();
    do_moves_and_check_validity(&mut board, &move_strings);

    let mut moves = vec![];
    board.generate_moves(&mut moves);
    assert!(!moves.contains(&board.move_from_san("2b5>11").unwrap()));
    // plays_correct_move_property(&moves_strings, TacticAnswer::AvoidMoves(&["2b5>1"]));
}

/// The correct answer to a tactic can be either a lost of winning/non-losing moves, or simply a list of moves to specifically avoid
enum TacticAnswer {
    AvoidMoves(&'static [&'static str]),
    PlayMoves(&'static [&'static str]),
}

fn plays_correct_move_property(move_strings: &[&str], correct_moves: TacticAnswer) {
    let mut board = Board::default();
    let mut moves = vec![];

    do_moves_and_check_validity(&mut board, move_strings);

    board.generate_moves(&mut moves);
    let mut mcts = mcts::Tree::new_root();

    let relevant_moves = match correct_moves {
        TacticAnswer::AvoidMoves(a) | TacticAnswer::PlayMoves(a) => a,
    };

    for move_string in relevant_moves {
        assert_eq!(
            *move_string,
            board.move_to_san(&board.move_from_san(move_string).unwrap())
        );
        assert!(
            moves.contains(&board.move_from_san(move_string).unwrap()),
            "Candidate move {} was not among legal moves {:?} on board\n{:?}",
            move_string,
            moves,
            board
        );
    }
    let mut moves = vec![];
    let mut simple_moves = vec![];

    for i in 1..50000 {
        mcts.select(&mut board.clone(), &mut simple_moves, &mut moves);
        if i % 10000 == 0 {
            let (best_move, _score) = mcts.best_move(0.1);
            match correct_moves {
                TacticAnswer::AvoidMoves(moves) =>
                    assert!(moves
                        .iter()
                        .all(|mv| best_move != board.move_from_san(mv).unwrap()),
                            "{} played {}, one of the losing moves {:?} after {} iterations on board:\n{:?}",
                            board.side_to_move(), board.move_to_san(&best_move), moves, i, board),
                TacticAnswer::PlayMoves(moves) =>
                    assert!(moves
                                .iter()
                                .any(|mv| best_move == board.move_from_san(mv).unwrap()),
                            "{} didn't play one of {:?} to avoid loss, {} played instead after {} iterations on board:\n{:?}",
                            board.side_to_move(), moves, board.move_to_san(&best_move), i, board),
            }
        }
    }
}
