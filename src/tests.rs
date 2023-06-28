use crate::utils;
use crate::eval;
use crate::move_gen;
use crate::utils::{VectorReuser, generate_into, Square, Move};
use crate::utils::*;
use crate::move_gen::*;
use crate::hashing::Hasher;


use std::collections::HashMap;
use crate::utils::Move::{Castling, KingMove, PawnPush, RookMove};

pub fn main() {
    // first_test();
    // second_test();
    // third_test();
    // fourth_test();
    // fifth_test();
    // sixth_test();
    // seventh_test();
    // eighth_test();
    // unsafe {
    //     tenth_test();
    // }
    // unsafe {
    //     eleventh_test();
    // }
    // twelfth_test();
    // thentyfirsttest()
    perftest();
    // twentysecondtest();
    // twentieth_test();
    // thirteenth_test();
    // fourteenth_test();
    // eighteenth_test();
    // nineteenth_test();
    // fifteenth_test();
    // sixteenth_test();
    // let board = utils::Board::from_fen("rnbqkbnr/pppp1ppp/8/8/3Pp3/N3P3/PPP2PPP/R1BQKBNR b KQkq d3 0 3");
    // let mut vector = Vec::new();
    // let mut instant = Instant::now();
    // for _ in 0..100_000_000 {
    //     sort_benchmark(&board, &mut vector);
    //     if let Some(utils::Move::Castling(a, b, c, d)) = vector.get(0) {
    //         println!("{} {} {} {}", a, b, c, d);
    //     }
    // }
    // println!("{}", instant.elapsed().as_secs_f64());
    // let mut new_instant = Instant::now();
    // for _ in 0..100_000_000 {
    //     unstable_sort_benchmark(&board, &mut vector);
    //     if let Some(utils::Move::Castling(a, b, c, d)) = vector.get(0) {
    //         println!("{} {} {} {}", a, b, c, d);
    //     }
    // }
    // println!("{}", new_instant.elapsed().as_secs_f64());
}

fn epgentest() {
    let mut b = utils::Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
    // let m = Move::EnPassant()
    println!("{}", b.start_perf(1));
}


// Perft(1) = 20 in 0.001312 sec
// Perft(2) = 400 in 0.001263 sec
// Perft(3) = 8902 in 0.004491 sec
// Perft(4) = 197281 in 0.023202 sec
// Perft(5) = 4865609 in 0.419626 sec
// Perft(6) = 119060324 in 10.080624 sec
// Perft(7) = 3195901860 in 277.148570 sec
fn perftest() {
    // let mut b = utils::Board::from_starting_position();
    // let mut b = utils::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
    // let mut b = utils::Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");
    // let mut b = utils::Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ");
    // let mut b = utils::Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ");
    let mut b = utils::Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
    // b.make_move(&Castling(7, 4, 7, 6));
    // b.undo_move(&Castling(7, 4, 7, 6));
    // b.make_move(&KingMove(7, 4, 7, 5, Square::E));
    // b.undo_move(&KingMove(7, 4, 7, 5, Square::E));
    // b.make_move(&RookMove(7, 7, 7, 6, Square::E));
    // b.undo_move(&RookMove(7, 7, 7, 6, Square::E));
    // b.make_move(&PawnPush(6, 1, 5, 1));
    println!("{}", b.start_perf(1));
}

// fn twentysecondtest() {
//     // let fen = "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3";
//     // let fen = "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
//     let fen = "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1";
//     // let mut b = utils::Board::from_fen(fen);
//     let mut b = utils::Board::from_starting_position();
//     let hasher = Hasher::new(69420);
//     let eval = b.debug_compute_evaluation(&hasher, &mut HashMap::new());
//     println!("eval: {}", eval);
//     // b.make_move(&Move::PawnPush(1, 0, 2, 0));
//     // let choice = eval::top_level_search(&mut b, 0,7, 69420).unwrap();
//     println!("{} {:?}", choice.to_string(), choice);
// }
//
//
// fn thentyfirsttest() {
//     let hasher = Hasher::new(69420);
//     let mut pawn_cache = HashMap::with_capacity(30);
//     let mut board = Board::from_starting_position();
//     let mut vector = Vec::with_capacity(30);
//     generate_into(&board, &mut vector);
//     for i in vector {
//         board.make_move(&i);
//         println!("eval: '{}'; move: {:?} or {}", board.compute_evaluation(&hasher, &mut pawn_cache), i, i.to_string()   );
//         board.undo_move(&i);
//     }
// }
//
// fn twentieth_test() {
//     let hasher = Hasher::new(69420);
//     for i in hasher.get_hash_values() {
//         println!("{}", i);
//         break;
//     }
// }
//
//
// fn nineteenth_test() {
//     let mut board = Board::from_fen("r1bqkbnr/ppp2ppp/2n5/4p3/3pP3/1B3N2/PPPP1PPP/RNBQK2R w KQkq - 0 5");
//     let m1 = Move::TwoSquarePawnMove(6, 2, 4, 2);
//     let m2 = Move::EnPassant(4, 3, 5, 2, Square::PW);
//     let hasher = Hasher::new(10000);
//     let mut hashes = Vec::new();
//     hashes.push(hasher.get_zobrist_hash(&board));
//     board.display_debug_info();
//     board.make_move(&m1);
//     hashes.push(hasher.get_zobrist_hash(&board));
//     board.display_debug_info();
//     board.make_move(&m2);
//     hashes.push(hasher.get_zobrist_hash(&board));
//     board.display_debug_info();
//     board.undo_move(&m2);
//     hashes.push(hasher.get_zobrist_hash(&board));
//     board.display_debug_info();
//     board.undo_move(&m1);
//     hashes.push(hasher.get_zobrist_hash(&board));
//     board.display_debug_info();
//     println!("{:#?}", hashes);
// }
//
// fn eighteenth_test() {
//     let en_string = "r1bqkbnr/ppp2ppp/2n5/4p3/2PpP3/1B3N2/PP1P1PPP/RNBQK2R b KQkq c3 0 5";
//     let fen_string = "r1bqkbnr/ppp2ppp/2n5/4p3/2PpP3/1B3N2/PP1P1PPP/RNBQK2R b KQkq - 0 5";
//     let hasher = Hasher::new(10000);
//     let hash1 = hasher.get_zobrist_hash(&Board::from_fen(en_string));
//     let hash2 = hasher.get_zobrist_hash(&Board::from_fen(fen_string));
//     assert_ne!(hash1, hash2);
// }
//
// fn seventeenth_test() {
//     let fen_string = "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3";
//     let mut b = utils::Board::from_fen(fen_string);
//     // b.make_move(&Move::PawnPush(1, 0, 2, 0));
//     // let choice = eval::top_level_search(&mut b, 0,4, 69420).unwrap();
//     // println!("{:?}", choice);
// }
//
//
// fn sixteenth_test() {
//     let fen_string = "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
//     let b = utils::Board::from_fen(fen_string);
//     println!("{}", fen_string);
//     println!("{}", b.to_fen());
//     let bb = utils::Board::from_fen(&b.to_fen());
//     assert_eq!(b.to_fen(), bb.to_fen());
// }


// fn fifteenth_test() {
//     // let fen_string = "rnbqkb1r/pppp1ppp/8/1N2p2n/2B1P3/8/PPPP1PPP/R1B1K1NR w KQkq - 0 1";
//     let fen_string = "1rb1kb1r/p1p2Bpp/8/4p2n/8/8/PPP2PPP/RNq1K1NR w KQk -";
//     let mut b = utils::Board::from_fen(fen_string);
//     let mut vector = Vec::new();
//     generate_into(&b, &mut vector);
//     let mut hash = b.get_hash();
//     b.display_board();
//     b.display_debug_info();
//     println!("------------------");
//     for i in vector.iter() {
//         println!("{:?}", i);
//         b.make_move(i);
//         b.undo_move(i);
//         if hash != b.get_hash() {
//             b.display_board();
//             b.display_debug_info();
//             panic!("cock");
//         }
//     }
// }



// fn fourteenth_test() {
//     let fen_string = "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3";
//     let mut b = utils::Board::from_fen(fen_string);
//     let choice = eval::top_level_search(&mut b, 0,6, 69420).unwrap();
//     // 6 - 2 s
//     // 7 - 10 s
//     // 8 - 1 min
//     println!("{:?}", choice);
// }


// fn thirteenth_test() {
//     let b = utils::Board::from_fen("r1bqkb1r/pppp1ppp/2n4n/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4");
//     // let mut b = utils::Board::from_starting_position();
//     println!("{}", evaluation(&b));
//     b.display_board();
// }

// fn twelfth_test() {
//     let fen_string = "rnb1kbnr/pppp1ppp/8/4p3/4q3/8/PPPPKPPP/RNBQ1BNR b kq - 0 4";
//     let mut b = utils::Board::from_fen(fen_string);
//     // b.display_board();
//     let eval = eval::search(&mut VectorReuser::new(), &mut b, 1, 0, f64::MIN, f64::MAX, &mut HashMap::with_capacity(100_000));
//     println!("{:?}", eval);
//     let fen_string = "r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 3 3";
//     // let fen_string = "r1bqkb1r/pppp1ppp/2n4n/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4";
//     let mut b = utils::Board::from_fen(fen_string);
//     // b.display_board();
//     let eval = eval::search(&mut VectorReuser::new(), &mut b, 5, 0, f64::MIN, f64::MAX, &mut HashMap::with_capacity(100_000));
//     println!("{:?}", eval);
// }


//
// unsafe fn eleventh_test() {
//     let fen_string = "r1bq1rk1/pp3ppp/1npbpn2/3p4/3P4/2PBPNB1/PP3PPP/RN1Q1RK1 w - - 3 9";
//     let mut b = utils::Board::from_fen(fen_string);
//     let mut vector = Vec::new();
//     generate_into(&b, &mut vector);
//     for (i, j) in vector.iter().enumerate() {
//         b.make_move(&j);
//         println!("{} {:?}", i + 1, j);
//         b.undo_move(&j);
//     }
// }

//
//
// unsafe fn tenth_test() {
//     let mut b = utils::Board::from_starting_position();
//     let mut bb = &mut b as *mut utils::Board;
//     b.make_move(&utils::Move::TwoSquarePawnMove(6, 4, 4, 4));
//     b.make_move(&utils::Move::Normal(1, 4, 2, 4, utils::Square::E));
//     b.make_move(&utils::Move::Normal(4, 4, 3, 4, utils::Square::E));
//     b.make_move(&utils::Move::TwoSquarePawnMove(1, 3, 3, 4));
//     b.display_board();
//     // println!("{:?}", b);
//     for i in move_gen::PawnMoveGenerator::new(&*bb, 3, 4) {
//         println!("{:?}", i);
//         b.make_move(&i);
//         b.display_board();
//         b.undo_move(&i);
//         b.display_board();
//     }
//     println!("doing it again");
//     for i in move_gen::PawnMoveGenerator::new(&*bb, 3, 4) {
//         println!("{:?}", i);
//         b.make_move(&i);
//         b.display_board();
//         b.undo_move(&i);
//         b.display_board();
//     }
// }
//
//
// fn ninth_test() {
//     let fen_string = "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
//     let mut b = utils::Board::from_fen(fen_string);
//     b.display_board();
//     println!("-------------------------");
//     let mut g = move_gen::PawnMoveGenerator::new(&b, 3, 4);
//     for i in g {
//         println!("{:?}", i);
//     }
// }
// fn eighth_test() {
//     // KingMove(7, 4, 7, 5, E)
//     // KingMove(7, 4, 6, 4, E)
//     // Castling(7, 4, 7, 6)
//     let fen_string = "r1bqkb1r/1ppp1ppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 2 5";
//     let mut b = utils::Board::from_fen(fen_string);
//     b.display_board();
//     println!("-------------------------");
//     let mut g = move_gen::KingMoveGenerator::new(&b, 7, 4);
//     for i in g {
//         println!("{:?}", i);
//     }
// }
//
//
// fn seventh_test() {
//     // KingMove(3, 3, 4, 3, E)
//     // KingMove(3, 3, 3, 4, P { white: false })
//     // KingMove(3, 3, 2, 3, E)
//     // KingMove(3, 3, 3, 2, E)
//     // KingMove(3, 3, 4, 2, E)
//     // KingMove(3, 3, 2, 4, E)
//     // KingMove(3, 3, 2, 2, E)
//     let fen_string = "rnbq1bnr/pppp1ppp/8/3Kp3/4Pk2/8/PPPP1PPP/RNBQ1BNR w - - 10 7";
//     let mut b = utils::Board::from_fen(fen_string);
//     b.display_board();
//     println!("-------------------------");
//     let mut g = move_gen::KingMoveGenerator::new(&b, 3, 3);
//     for i in g {
//         println!("{:?}", i);
//     }
// }
//
//
// fn sixth_test() {
//     // Normal(3, 3, 4, 4, E)
//     // Normal(3, 3, 5, 5, E)
//     // Normal(3, 3, 6, 6, P { white: true })
//     // Normal(3, 3, 4, 2, E)
//     // Normal(3, 3, 5, 1, E)
//     // Normal(3, 3, 6, 0, P { white: true })
//     // Normal(3, 3, 2, 4, E)
//     // Normal(3, 3, 2, 2, E)
//     // Normal(3, 3, 4, 3, E)
//     // Normal(3, 3, 5, 3, E)
//     // Normal(3, 3, 6, 3, P { white: true })
//     // Normal(3, 3, 2, 3, E)
//     // Normal(3, 3, 1, 3, E)
//     // Normal(3, 3, 0, 3, E)
//     // Normal(3, 3, 3, 4, E)
//     // Normal(3, 3, 3, 5, E)
//     // Normal(3, 3, 3, 6, E)
//     // Normal(3, 3, 3, 7, E)
//     // Normal(3, 3, 3, 2, E)
//     // Normal(3, 3, 3, 1, E)
//     // Normal(3, 3, 3, 0, E)
//     let fen_string = "rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 1 3";
//     let mut b = utils::Board::from_fen(fen_string);
//     b.display_board();
//     println!("-------------------------");
//     let mut g = move_gen::QueenMoveGenerator::new(&b, 3, 3);
//     for i in g {
//         println!("{:?}", i);
//     }
// }
//
//
// fn fifth_test() {
//     // RookMove(7, 4, 6, 4, E)
//     // RookMove(7, 4, 5, 4, E)
//     // RookMove(7, 4, 4, 4, R { white: false })
//     // RookMove(7, 4, 7, 5, E)
//     // RookMove(7, 4, 7, 6, E)
//     // RookMove(7, 4, 7, 7, E)
//     // RookMove(7, 4, 7, 3, E)
//     // RookMove(7, 4, 7, 2, E)
//     // RookMove(7, 4, 7, 1, E)
//     // RookMove(7, 4, 7, 0, E)
//     let fen_string = "8/2pk3p/p5r1/1p6/1R1Pr1P1/8/P2K1P1P/4R3 w - - 0 23";
//     let mut b = utils::Board::from_fen(fen_string);
//     let mut vector = Vec::new();
//     generate_rook_moves(&b, 7, 4, &mut vector);
//     vector.sort();
//     // b.display_board();
//     println!("-------------------------");
//     let mut g: Vec<_> = RookMoveGenerator::new(&b, 7, 4).collect();
//     g.sort();
//     for (i, j) in vector.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
//     for (i, j) in g.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
// }
//
//
// fn fourth_test() {
//     // Normal(4, 4, 5, 5, E)
//     // Normal(4, 4, 6, 6, E)
//     // Normal(4, 4, 5, 3, E)
//     // Normal(4, 4, 6, 2, E)
//     // Normal(4, 4, 3, 5, E)
//     // Normal(4, 4, 2, 6, E)
//     // Normal(4, 4, 1, 7, P { white: false })
//     // Normal(4, 4, 3, 3, E)
//     // Normal(4, 4, 2, 2, N { white: false })
//     let fen_string = "r2qkb1r/pbpp1ppp/1pn2n2/4p3/4B3/2P1P1P1/PP1P1P1P/RNBQK1NR w KQkq - 2 6";
//     let mut b = utils::Board::from_fen(fen_string);
//     let mut vector = Vec::new();
//     generate_bishop_moves(&b, 4, 4, &mut vector);
//     vector.sort();
//     b.display_board();
//     println!("-------------------------");
//     let mut g: Vec<_> = BishopMoveGenerator::new(&b, 4, 4).collect();
//     g.sort();
//     for (i, j) in vector.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
//     for (i, j) in g.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
// }
//
//
// fn third_test() {
//     // Normal(3, 4, 5, 5, E)
//     // Normal(3, 4, 5, 3, E)
//     // Normal(3, 4, 1, 5, P { white: false })
//     // Normal(3, 4, 1, 3, P { white: false })
//     // Normal(3, 4, 4, 6, E)
//     // Normal(3, 4, 4, 2, E)
//     // Normal(3, 4, 2, 6, E)
//     // Normal(3, 4, 2, 2, E)
//     let fen_string = "rnbqkb1r/pppppppp/8/3nN3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 8 5";
//     let mut vector = Vec::new();
//     let mut b = utils::Board::from_fen(fen_string);
//     generate_knight_moves(&b, 3, 4, &mut vector);
//     b.display_board();
//     println!("-------------------------");
//     let mut g: Vec<_> = KnightMoveGenerator::new(&b, 3, 4).collect();
//     vector.sort();
//     g.sort();
//     for (i, j) in vector.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
//     for (i, j) in g.iter().enumerate() {
//         println!("{} {:?}", i, j);
//     }
// }
//
//
// fn second_test() {
//     let fen_string = "4rrk1/5ppp/p1pb4/1p1n4/3P2b1/1BPQB1Pq/PP1N1P1P/R3R1K1 b - - 6 17";
//     let mut b = utils::Board::from_fen(fen_string);
//     b.display_board();
//     println!("-------------------------");
//     let m1 = utils::Move::Normal(5, 7, 3, 7, utils::Square::E);
//     let m2 = utils::Move::Normal(6, 0, 4, 0, utils::Square::E);
//     let m3 = utils::Move::Normal(0, 4, 2, 4, utils::Square::E);
//     b.make_move(&m1);
//     b.make_move(&m2);
//     b.make_move(&m3);
//     b.display_board();
//     println!("-------------------------");
//     b.undo_move(&m3);
//     b.undo_move(&m2);
//     b.undo_move(&m1);
//     b.display_board()
// }
//
//
// fn first_test() {
//     let mut b = utils::Board::from_starting_position();
//     b.display_board();
//     println!("-------------------------");
//     let m = utils::Move::Normal(6, 4, 4, 4, utils::Square::E);
//     b.make_move(&m);
//     b.display_board();
//     println!("-------------------------");
//     b.undo_move(&m);
//     b.display_board();
// }
//
//
