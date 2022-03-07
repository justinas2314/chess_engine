use crate::utils::{Board, Square};

pub fn evaluation(board: &Board) -> f64 {
    let mut total = 0.0;
    for i in 0..8 {
        for j in 0..8 {
            match board.board[i][j] {
                Square::E => (),
                Square::P {white: true } => {total += 1.0;},
                Square::P {white: false } => {total -= 1.0;},
                Square::N {white: true } => {total += 3.0;},
                Square::N {white: false } => {total -= 3.0;},
                Square::B {white: true } => {total += 3.0;},
                Square::B {white: false } => {total -= 3.0;},
                Square::R {white: true } => {total += 5.0;},
                Square::R {white: false } => {total -= 5.0;},
                Square::Q {white: true } => {total += 9.0;},
                Square::Q {white: false } => {total -= 9.0;},
                Square::K {white: true } => {total += 10_000.0;},
                Square::K {white: false } => {total -= 10_000.0;}
            }
        }
    }
    if board.white_to_move {
        total
    } else {
        -total
    }
}
