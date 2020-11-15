#[cfg(test)]
mod tests;

pub mod playtak;
pub mod uti;

use std::io::{Read, Write};
use std::{io, time};

use board_game_traits::board::{Board as BoardTrait, EvalBoard};
use board_game_traits::board::{Color, GameResult};
use pgn_traits::pgn::PgnBoard;

use taik::board;
use taik::board::Board;
use taik::board::TunableBoard;
use taik::mcts;
use taik::minmax;
use taik::pgn_writer::Game;
#[cfg(feature = "constant-tuning")]
use taik::tune::play_match::play_match_between_params;

fn main() {
    println!("play: Play against the mcts AI");
    println!("aimatch: Watch the minmax and mcts AIs play");
    println!("analyze: Mcts analysis of a position, provided from a simple move list");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words = input.split_whitespace().collect::<Vec<_>>();
    match words[0] {
        "play" => {
            let board = Board::default();
            play_human(board);
        }
        "aimatch" => {
            for i in 1..10 {
                mcts_vs_minmax(3, 50000 * i);
            }
        }
        "analyze" => test_position(),
        "game" => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();

            let games = taik::pgn_parser::parse_pgn(&input).unwrap();
            if games.is_empty() {
                panic!("Couldn't parse any games")
            }

            analyze_game(games[0].clone());
        }
        "mem_usage" => mem_usage(),
        "bench" => bench(),
        "selfplay" => mcts_selfplay(time::Duration::from_secs(10)),
        #[cfg(feature = "constant-tuning")]
        "play_params" => {
            #[allow(clippy::unreadable_literal)]
            let value_params1: &'static [f32] = &[
                0.054227155,
                0.3407015,
                0.4347485,
                0.54618615,
                0.5894169,
                0.41717935,
                0.80713177,
                1.6106186,
                1.3977867,
                1.6436608,
                2.0145588,
                0.8530996,
                -0.9235043,
                -0.5978478,
                -0.31175753,
                0.14952391,
                0.77818716,
                1.5191432,
                1.3946671,
                2.035646,
                0.981081,
                0.24216132,
                1.2395397,
                1.0178914,
                -2.203359,
                -1.7674192,
                -0.7277705,
                0.64038795,
                2.176997,
                -0.04819244,
                0.91904986,
                -1.266337,
                -0.828557,
                -0.42983347,
                0.080568284,
                0.69053686,
            ];
            #[allow(clippy::unreadable_literal)]
            let policy_params1: &'static [f32] = &[
                -3.9616194,
                -3.4906785,
                -3.277753,
                -2.7917902,
                -2.6880484,
                -2.9846509,
                -5.028032,
                -5.2466316,
                -4.9179077,
                -4.7460146,
                -4.6174083,
                -3.8573232,
                -4.1148667,
                -4.5389056,
                -4.1252546,
                -3.9228675,
                -2.4650762,
                1.3357767,
                0.9857822,
                0.051044937,
                1.1140109,
                -0.09581065,
                0.25960785,
                -4.472624,
                0.8161406,
                0.53994584,
                0.7810427,
                1.5053948,
            ];
            let value_params2 = Board::VALUE_PARAMS;
            let policy_params2 = Board::POLICY_PARAMS;
            play_match_between_params(value_params1, value_params2, policy_params1, policy_params2);
        }
        s => println!("Unknown option \"{}\"", s),
    }
}

fn mcts_selfplay(max_time: time::Duration) {
    let mut board = Board::default();
    let mut moves = vec![];

    let mut white_elapsed = time::Duration::default();
    let mut black_elapsed = time::Duration::default();

    while board.game_result().is_none() {
        let start_time = time::Instant::now();
        let (best_move, score) = mcts::play_move_time(board.clone(), max_time);

        match board.side_to_move() {
            Color::White => white_elapsed += start_time.elapsed(),
            Color::Black => black_elapsed += start_time.elapsed(),
        }

        board.do_move(best_move.clone());
        moves.push(best_move.clone());
        println!(
            "{:6}: {:.3}, {:.1}s",
            best_move,
            score,
            start_time.elapsed().as_secs_f32()
        );
        io::stdout().flush().unwrap();
    }

    println!(
        "{:.1} used by white, {:.1} for black",
        white_elapsed.as_secs_f32(),
        black_elapsed.as_secs_f32()
    );

    print!("\n[");
    for mv in moves.iter() {
        print!("\"{:?}\", ", mv);
    }
    println!("]");

    for (ply, mv) in moves.iter().enumerate() {
        if ply % 2 == 0 {
            print!("{}. {:?} ", ply / 2 + 1, mv);
        } else {
            println!("{:?}", mv);
        }
    }
    println!();

    println!("\n{:?}\nResult: {:?}", board, board.game_result());
}

fn mcts_vs_minmax(minmax_depth: u16, mcts_nodes: u64) {
    println!("Minmax depth {} vs mcts {} nodes", minmax_depth, mcts_nodes);
    let mut board = Board::default();
    let mut moves = vec![];
    while board.game_result().is_none() {
        let num_moves = moves.len();
        if num_moves > 10 && (1..5).all(|i| moves[num_moves - i] == moves[num_moves - i - 4]) {
            break;
        }
        match board.side_to_move() {
            Color::Black => {
                let (best_move, score) = mcts::mcts(board.clone(), mcts_nodes);
                board.do_move(best_move.clone());
                moves.push(best_move.clone());
                println!("{:6}: {:.3}", best_move, score);
                io::stdout().flush().unwrap();
            }

            Color::White => {
                let (best_move, score) = minmax::minmax(&mut board, minmax_depth);
                board.do_move(best_move.clone().unwrap());
                moves.push(best_move.clone().unwrap());
                print!("{:6}: {:.2}, ", best_move.unwrap(), score);
                io::stdout().flush().unwrap();
            }
        }
    }
    print!("\n[");
    for mv in moves.iter() {
        print!("\"{:?}\", ", mv);
    }
    println!("]");

    for (ply, mv) in moves.iter().enumerate() {
        if ply % 2 == 0 {
            print!("{}. {:?} ", ply / 2 + 1, mv);
        } else {
            println!("{:?}", mv);
        }
    }
    println!();

    println!("\n{:?}\nResult: {:?}", board, board.game_result());
}

fn test_position() {
    let mut board = Board::default();
    let mut moves = vec![];

    println!("Enter moves:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for mv_san in input.split_whitespace() {
        let mv = board.move_from_san(&mv_san).unwrap();
        board.generate_moves(&mut moves);
        assert!(moves.contains(&mv));
        board.do_move(mv);
        moves.clear();
    }

    println!("{:?}", board);

    let mut simple_moves = vec![];
    let mut moves = vec![];

    board.generate_moves_with_probabilities(&mut simple_moves, &mut moves);
    moves.sort_by_key(|(_mv, score)| -(score * 1000.0) as i64);

    println!("Top 10 heuristic moves:");
    for (mv, score) in moves.iter().take(10) {
        println!("{}: {:.3}", mv, score);
        let mut coefficients = vec![0.0; Board::POLICY_PARAMS.len()];
        board.coefficients_for_move(&mut coefficients, mv, moves.len());
        for coefficient in coefficients {
            print!("{:.1}, ", coefficient);
        }
        println!();
    }

    for d in 1..=3 {
        let (best_move, score) = minmax::minmax(&mut board, d);

        println!(
            "Depth {}: minmax played {:?} with score {}",
            d, best_move, score
        );
    }

    let mut tree = mcts::RootNode::new(board.clone());
    for i in 1.. {
        tree.select();
        if i % 100_000 == 0 {
            println!(
                "{} visits, val={:.4}, static eval={:.4}, static winning probability={:.4}",
                tree.visits(),
                tree.mean_action_value(),
                board.static_eval(),
                mcts::cp_to_win_percentage(board.static_eval())
            );
            tree.print_info();
            println!("Best move: {:?}", tree.best_move())
        }
    }
}

fn analyze_game(game: Game<Board>) {
    let mut board = game.start_board.clone();
    let mut ply_number = 2;
    for (mv, _) in game.moves {
        board.do_move(mv.clone());
        if board.game_result().is_some() {
            break;
        }
        let (best_move, score) = mcts::mcts(board.clone(), 1_000_000);
        if ply_number % 2 == 0 {
            print!(
                "{}. {}: {{{:.2}%, best reply {}}} ",
                ply_number / 2,
                board.move_to_san(&mv),
                (1.0 - score) * 100.0,
                best_move
            );
        } else {
            println!(
                "{}... {}: {{{:.2}%, best reply {}}}",
                ply_number / 2,
                board.move_to_san(&mv),
                (1.0 - score) * 100.0,
                best_move
            );
        }
        ply_number += 1;
    }
}

/// Play a game against the engine through stdin
fn play_human(mut board: Board) {
    match board.game_result() {
        None => {
            use board_game_traits::board::Color::*;
            println!("Board:\n{:?}", board);
            // If black, play as human
            if board.side_to_move() == Black {
                println!("Type your move in algebraic notation (c3):");

                let reader = io::stdin();
                let mut input_str = "".to_string();
                let mut legal_moves = vec![];
                board.generate_moves(&mut legal_moves);
                // Loop until user enters a valid move
                loop {
                    input_str.clear();
                    reader
                        .read_line(&mut input_str)
                        .expect("Failed to read line");

                    match board.move_from_san(input_str.trim()) {
                        Ok(val) => {
                            if legal_moves.contains(&val) {
                                break;
                            }
                            println!("Move {:?} is illegal! Legal moves: {:?}", val, legal_moves);
                            println!("Try again: ");
                        }

                        Err(error) => {
                            println!("{}, try again.", error);
                        }
                    }
                }
                let c_move = board.move_from_san(input_str.trim()).unwrap();
                board.do_move(c_move);
                play_human(board);
            } else {
                let (best_move, score) = mcts::mcts(board.clone(), 1_000_000);

                println!("Computer played {:?} with score {}", best_move, score);
                board.do_move(best_move);
                play_human(board);
            }
        }

        Some(GameResult::WhiteWin) => println!("White won! Board:\n{:?}", board),
        Some(GameResult::BlackWin) => println!("Black won! Board:\n{:?}", board),
        Some(GameResult::Draw) => println!("The game was drawn! Board:\n{:?}", board),
    }
}

fn bench() {
    const NODES: u64 = 1_000_000;
    let start_time = time::Instant::now();
    {
        let board = Board::default();

        let (_move, score) = mcts::mcts(board, NODES);
        print!("{:.3}, ", score);
    }

    {
        let mut board = Board::default();

        do_moves_and_check_validity(&mut board, &["d3", "c3", "c4", "1d3<", "1c4+", "Sc4"]);

        let (_move, score) = mcts::mcts(board, NODES);
        print!("{:.3}, ", score);
    }
    {
        let mut board = Board::default();

        do_moves_and_check_validity(
            &mut board,
            &[
                "c2", "c3", "d3", "b3", "c4", "1c2-", "1d3<", "1b3>", "1c4+", "Cc2", "a1", "1c2-",
                "a2",
            ],
        );

        let (_move, score) = mcts::mcts(board, NODES);
        println!("{:.3}", score);
    }
    let time_taken = start_time.elapsed();
    println!(
        "{} nodes in {} ms, {:.1} knps",
        NODES * 3,
        time_taken.as_millis(),
        NODES as f64 * 3.0 / (1000.0 * time_taken.as_secs_f64())
    );
}

/// Print memory usage of various data types in the project, for debugging purposes
fn mem_usage() {
    use std::mem;
    println!("Tak board: {} bytes", mem::size_of::<board::Board>());
    println!("Tak board cell: {} bytes", mem::size_of::<board::Stack>());
    println!("Tak move: {} bytes", mem::size_of::<board::Move>());

    println!("MCTS node: {} bytes.", mem::size_of::<mcts::Tree>());
    println!("MCTS edge: {} bytes", mem::size_of::<mcts::TreeEdge>());
    let board = board::Board::default();
    let mut tree = mcts::RootNode::new(board);
    for _ in 0..2 {
        tree.select();
    }
    println!(
        "MCTS node's children: {} bytes.",
        tree.children().len() * mem::size_of::<mcts::Tree>()
    );
}

fn do_moves_and_check_validity(board: &mut Board, move_strings: &[&str]) {
    let mut moves = vec![];
    for mv_san in move_strings.iter() {
        let mv = board.move_from_san(&mv_san).unwrap();
        board.generate_moves(&mut moves);
        assert!(
            moves.contains(&mv),
            "Move {} was not among legal moves: {:?}\n{:?}",
            board.move_to_san(&mv),
            moves,
            board
        );
        board.do_move(mv);
        moves.clear();
    }
}
