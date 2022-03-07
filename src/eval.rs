use crate::utils::{Move, Board, VectorReuser, HashMapReuser, generate_into, generate_captures_into};
use crate::hashing::Hasher;
use crate::evaluation_function::evaluation;
use crate::move_gen::generate_defenders_into;
use std::cmp::max;

pub fn top_level_search(board: &mut Board, depth: usize, seed: u64) -> Option<Move> {
    let mut vector = Vec::new();
    let mut reuser = VectorReuser::new();
    let mut cache = HashMapReuser::new();
    let hasher = Hasher::new(seed);
    let mut alpha = -100_000.0;
    let mut best_move = None;
    let mut temp_score;
    generate_into(&board, &mut vector);
    vector.sort_by(|a, b| b.cmp(a));
    for i in vector {
        board.make_move(&i);
        temp_score = -search(&mut reuser, board, depth - 1, 0, -100_000.0, -alpha, &mut cache, &hasher);
        if temp_score > alpha {
            best_move = Some(i.clone());
            alpha = temp_score;
        }
        board.undo_move(&i);
    }
    best_move
}


pub fn search(reuser: &mut VectorReuser<Move>, board: &mut Board, depth: usize, index: usize, mut alpha: f64, beta: f64, cache: &mut HashMapReuser<u64, f64>, hasher: &Hasher) -> f64 {
    let hash;
    let mut eval = evaluation(board);
    let vector_pointer;
    if eval < -1_000.0 {
        return eval;
    }
    // hash = board.get_hash();
    hash = hasher.get_zobrist_hash(&board);
    if let Some(x) = cache.get(index, &hash) {
        return *x;
    }
    if index >= depth {
        eval = quint_search(reuser, board, index, alpha, beta);
        cache.insert(index, hash, eval);
        return eval;
    }
    vector_pointer = reuser.get_vector_pointer(index);
    unsafe {
        generate_into(board, &mut *vector_pointer);
        (*vector_pointer).sort_by(|a, b| b.cmp(a));
        for i in (*vector_pointer).iter() {
            board.make_move(i);
            eval = -search(reuser, board, depth, index + 1, -beta, -alpha, cache, hasher);
            board.undo_move(i);
            if eval > alpha {
                if eval >= beta {
                    return beta
                }
                alpha = eval;
            }
        }
    }
    cache.insert(index, hash, alpha);
    alpha
}
//
// pub fn old_search(reuser: &mut VectorReuser<Move>, board: &mut Board, depth: usize, index: usize, mut alpha: f64, beta: f64, cache: &mut HashMap<u64, f64>, hasher: &Hasher) -> f64 {
//     let hash;
//     let mut eval = evaluation(board);
//     let vector_pointer;
//     if eval < -1_000.0 {
//         return eval;
//     }
//     // hash = board.get_hash();
//     hash = hasher.get_zobrist_hash(&board);
//     if let Some(x) = cache.get(&hash) {
//         return *x;
//     }
//     if index >= depth {
//         eval = quint_search(reuser, board, index, alpha, beta);
//         cache.insert(hash, eval);
//         return eval;
//     }
//     vector_pointer = reuser.get_vector_pointer(index);
//     unsafe {
//         generate_into(board, &mut *vector_pointer);
//         (*vector_pointer).sort_by(|a, b| b.cmp(a));
//         for i in (*vector_pointer).iter() {
//             board.make_move(i);
//             eval = -old_search(reuser, board, depth, index + 1, -beta, -alpha, cache, hasher);
//             board.undo_move(i);
//             if eval > alpha {
//                 if eval >= beta {
//                     cache.insert(hash, beta);
//                     return beta
//                 }
//                 alpha = eval;
//             }
//         }
//     }
//     cache.insert(hash, alpha);
//     alpha
// }


// https://www.chessprogramming.org/Quiescence_Search
fn quint_search(reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize, mut alpha: f64, beta: f64) -> f64 {
    let mut eval= evaluation(board);
    let vector_pointer;
    if eval < -1_000.0 {
        return eval;
    }
    if eval > alpha {
        if eval >= beta {
            return beta;
        }
        alpha = eval;
    }
    vector_pointer = reuser.get_vector_pointer(index);
    unsafe {
        generate_captures_into(board, &mut *vector_pointer);
        (*vector_pointer).sort_by(|a, b| b.cmp(a));
        for i in (*vector_pointer).iter() {
            if see_capture(i, reuser, board, index + 1) < 0 {
                continue
            }
            board.make_move(i);
            eval = -quint_search(reuser, board, index + 1, -beta, -alpha);
            board.undo_move(i);
            if eval > alpha {
                if eval >= beta {
                    return beta
                }
                alpha = eval;
            }
        }
    }
    alpha
}


fn see_capture(capture: &Move, reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize) -> isize {
    let mut output = 0;
    let (x, y) = capture.get_xy();
    if let Some(worth) = capture.get_captured_piece_worth() {
        board.make_move(&capture);
        output = worth - static_exchange_evaluation(reuser, board, index, x, y);
        board.undo_move(&capture);
    }
    output
}


// https://www.chessprogramming.org/Static_Exchange_Evaluation
fn static_exchange_evaluation(reuser: &mut VectorReuser<Move>, board: &mut Board, index: usize, x: usize, y: usize) -> isize {
    let mut output = 0 ;
    let vector_pointer = reuser.get_vector_pointer(index);
    let move_with_smallest_piece;
    unsafe {
        generate_defenders_into(board, &mut *vector_pointer, x, y);
        if (*vector_pointer).len() == 0 {
            return output
        }
        move_with_smallest_piece = (*vector_pointer).iter().max().unwrap().clone();
        if let Some(worth) = move_with_smallest_piece.get_captured_piece_worth() {
            board.make_move(&move_with_smallest_piece);
            output = max(0, worth - static_exchange_evaluation(reuser, board, index + 1, x, y));
            board.undo_move(&move_with_smallest_piece);
        }
    }
    output
}
