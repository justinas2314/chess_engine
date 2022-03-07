use crate::utils::{Move, Board, Square, VectorReuser, generate_into, generate_captures_into};
use std::cmp::{max, min};
use std::collections::HashMap;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Eval {
    WhiteIsMated,
    Material(isize),
    BlackIsMated
}


impl Eval {
    pub fn is_mate(&self) -> bool {
        match self {
            Eval::WhiteIsMated | Eval::BlackIsMated => true,
            _ => false
        }
    }
}


pub fn evaluation(board: &Board) -> Eval {
    let mut total = 0;
    let mut white_king_exists = false;
    let mut black_king_exists = false;
    for i in 0..8 {
        for j in 0..8 {
            match board.board[i][j] {
                Square::E => (),
                Square::P {white: true } => {total += 1;},
                Square::P {white: false } => {total -= 1;},
                Square::N {white: true } => {total += 3;},
                Square::N {white: false } => {total -= 3;},
                Square::B {white: true } => {total += 3;},
                Square::B {white: false } => {total -= 3;},
                Square::R {white: true } => {total += 5;},
                Square::R {white: false } => {total -= 5;},
                Square::Q {white: true } => {total += 9;},
                Square::Q {white: false } => {total -= 9;},
                Square::K {white: true } => {white_king_exists = true},
                Square::K {white: false } => {black_king_exists = true}
            }
        }
    }
    match (white_king_exists, black_king_exists) {
        (true, false) => Eval::BlackIsMated,
        (false, true) => Eval::WhiteIsMated,
        _ => Eval::Material(total)
    }
}

// pub fn eval_captures(reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize, mut alpha: Eval, mut beta: Eval, cache: &mut HashMap<u64, Eval>) -> Eval {
pub fn eval_captures(reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize, mut alpha: Eval, mut beta: Eval) -> Eval {
    let mut eval= evaluation(board);
    if board.white_to_move {
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
    } else {
        if eval <= alpha {
            return alpha;
        }
        if eval < beta {
            beta = eval;
        }
    }
    // let hash = board.get_hash();
    // if let Some(x) = cache.get(&hash) {
    //     return *x;
    // }
    let vector_pointer = reuser.get_vector_pointer(index);
    unsafe {
        generate_captures_into(board, &mut *vector_pointer);
        if (*vector_pointer).len() == 0 {
            return eval;
        }
        (*vector_pointer).sort_by(|a, b| b.cmp(a));
        if board.white_to_move {
            for i in (*vector_pointer).iter() {
                board.make_move(&i);
                // eval = max(eval, eval_captures(reuser, board, index + 1, alpha, beta, cache));
                alpha = max(alpha, eval_captures(reuser, board, index + 1, alpha, beta));
                board.undo_move(&i);
                if alpha >= beta {
                    return beta;
                }
            }
            // cache.insert(hash, alpha);
            alpha
        } else {
            for i in (*vector_pointer).iter() {
                board.make_move(&i);
                // eval = min(eval, eval_captures(reuser, board, index + 1, alpha, beta, cache));
                beta = min(beta, eval_captures(reuser, board, index + 1, alpha, beta));
                board.undo_move(&i);
                if beta <= alpha {
                    return alpha;
                }
            }
            // cache.insert(hash, beta);
            beta
        }
    }
}


pub fn eval_move(reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize, depth: usize, mut alpha: Eval, mut beta: Eval, cache: &mut HashMap<u64, Eval>) -> Eval {
    let mut eval;
    let hash = board.get_hash();
    if index == depth {
        // eval = eval_captures(reuser, board, index, alpha, beta, cache);
        eval = eval_captures(reuser, board, index, alpha, beta);
        cache.insert(hash, eval);
        return eval;
    } else if let Some(x) = cache.get(&hash) {
        return *x;
    }
    eval = evaluation(board);
    match eval {
        Eval::WhiteIsMated => return alpha,
        Eval::BlackIsMated => return beta,
        _ => ()
    }
    let vector_pointer = reuser.get_vector_pointer(index);
    unsafe {
        generate_into(board, &mut *vector_pointer);
        (*vector_pointer).sort_by(|a, b| b.cmp(a));
        if board.white_to_move {
            for i in (*vector_pointer).iter() {
                board.make_move(&i);
                alpha = max(alpha, eval_move(reuser, board, index + 1, depth, alpha, beta, cache));
                board.undo_move(&i);
                if alpha >= beta {
                    return beta;
                }
            }
            cache.insert(hash, alpha);
            alpha
        } else {
            for i in (*vector_pointer).iter() {
                board.make_move(&i);
                beta = min(beta, eval_move(reuser, board, index + 1, depth, alpha, beta, cache));
                board.undo_move(&i);
                if beta <= alpha {
                    return alpha;
                }
            }
            cache.insert(hash, beta);
            beta
        }
    }
}


// fn static_exchange_evaluation(board: &Board)