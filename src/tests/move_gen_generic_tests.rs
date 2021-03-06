use crate::position::Position;
use board_game_traits::Position as PositionTrait;

#[test]
fn start_position_move_gen_test() {
    start_position_move_gen_prop::<4>();
    start_position_move_gen_prop::<5>();
    start_position_move_gen_prop::<6>();
    start_position_move_gen_prop::<7>();
    start_position_move_gen_prop::<8>();
}

pub fn perft<const S: usize>(position: &mut Position<S>, depth: u16) -> u64 {
    if depth == 0 || position.game_result().is_some() {
        1
    } else {
        let mut moves = vec![];
        position.generate_moves(&mut moves);
        moves
            .into_iter()
            .map(|mv| {
                let old_position = position.clone();
                let reverse_move = position.do_move(mv.clone());
                let num_moves = perft(position, depth - 1);
                position.reverse_move(reverse_move);
                debug_assert_eq!(
                    *position, old_position,
                    "Failed to restore old board after {:?} on\n{:?}",
                    mv, old_position
                );
                num_moves
            })
            .sum()
    }
}

/// Verifies the perft result of a position against a known answer
pub fn perft_check_answers<const S: usize>(position: &mut Position<S>, answers: &[u64]) {
    for (depth, &answer) in answers.iter().enumerate() {
        assert_eq!(
            perft(position, depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(&mut position.flip_board_x(), depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(&mut position.flip_board_y(), depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(&mut position.flip_colors(), depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(&mut position.rotate_board(), depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(&mut position.rotate_board().rotate_board(), depth as u16),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
        assert_eq!(
            perft(
                &mut position.rotate_board().rotate_board().rotate_board(),
                depth as u16
            ),
            answer,
            "Wrong perft result on\n{:?}",
            position
        );
    }
}

fn start_position_move_gen_prop<const S: usize>() {
    let mut position = <Position<S>>::default();
    let mut moves = vec![];
    position.generate_moves(&mut moves);
    assert_eq!(moves.len(), S * S);
    for mv in moves {
        let reverse_move = position.do_move(mv);
        let mut moves = vec![];
        position.generate_moves(&mut moves);
        assert_eq!(moves.len(), S * S - 1);
        position.reverse_move(reverse_move);
    }
}
