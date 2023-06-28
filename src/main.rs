mod utils;
mod tests;
mod eval;
mod move_gen;
mod evaluation_function;
mod hashing;
mod debug;
mod mp_utils;

use crate::utils::{Board, PositionStorer};
use crate::eval::{SearchArgs, compute, HasherRef, PositionStorerRef};
use crate::hashing::Hasher;
use crate::mp_utils::mp_compute_wrapper;

use std::pin::Pin;
use std::cmp::min;
use std::{env, sync, thread};
use std::io::{Read, Write};
use std::net;
use std::str;
use std::time::Duration;


// 8/1Rp3p1/3bNk2/3P1p2/5P1p/5K2/3r2P1/8 b - - 0 1
// 8/1RN2kp1/3b4/3P1p2/5P1p/5K2/3r2P1/8 b - - 0 2
// ignoruoju MIN_DEPTH kai laiko tik tiek
pub const MIN_DEPTH_TIME_LIMIT: u64 = 120_000;
// teoriskai imanoma jei pozicija analizuosiu MIN_DEPTH_TIME_LIMIT laiko pasiflagginti
// bet muset taip nebus
pub const MIN_DEPTH: usize = 7;
pub const MAX_DEPTH: usize = 30;

fn alternative_main(white: bool, address: &str) {
    let mut sender_clone;
    let mut hasher_ref;
    let mut string = String::with_capacity(1024);
    let mut time_left;
    let mut board;
    let hasher = Pin::new(Box::new(Hasher::new(69420)));
    let mut prev_eval = 0;
    let (sender, receiver) = sync::mpsc::channel();
    let mut socket = net::TcpStream::connect(address).expect("blemba nedai, https://www.isitdownrightnow.com/localhost.html");
    if let Err(e) = socket.set_nodelay(true) {
        println!("Nepavyko ijungti no delay communication su serveriu: {}", e);
    }
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
        println!("Nepavyko pakeisti read timeout i 0.1 sekundes: {}", e);
    } else {
        println!("Read timeout yra 0.1 sekundes");
    }
    loop {
        if let Err(err) = socket.read_to_string(&mut string) {
            // nesvarbu
            // println!("nezinau ar svarbu, bet: {}", err);
        }
        if string.len() == 0 {
            continue;
        } else {
            println!("string: '{}'", string);
        }
        time_left = string
            .split(' ')
            .rev()
            .nth(if white {1} else {0})
            .unwrap()
            .parse::<u64>()
            .unwrap();
        board = Board::from_fen(&string);
        println!("time_left: {}", time_left);
        println!("computing position, fen: '{}'", string);
        sender_clone = sender.clone();
        // naudojama sena gera trust me bro technika
        unsafe {
            hasher_ref = HasherRef::new(&*hasher);
            thread::spawn(move || {
                compute(board, hasher_ref, prev_eval, min(10_000, time_left / 20), sender_clone);
            });
        }
        if let Ok((eval, choice)) = receiver.recv() {
            if let Some(actual_move) = choice {
                socket.write(actual_move.to_string().as_bytes()).expect("blemba nedai");
                println!("eval: {}; choice: {}", eval, actual_move.to_string());
                prev_eval = eval;
            } else {
                println!("eval: {}; choice: None", eval);
            }
        } else {
            println!("Something is sussy");
        }
        string.clear();
    }
    //
    // board.display_board();
    // if let Some(m) = top_level_search(&mut board, 5, 69420) {
    //     socket.write(m.to_string().as_bytes()).expect("blemba nedai");
    // } else {
    //     panic!("Pasiduodu");
    // }
    // // tests::main();
}
// TODO write a check to see if a seed does not generate two equal numbers
fn main() {
    // println!("ANTI-GASLIGHT PRINT");
    let mut sender_clone;
    let mut position_storer = Pin::new(Box::new(PositionStorer::new()));
    let mut string = String::with_capacity(1024);
    let args: Vec<String> = env::args().collect();
    let (sender, receiver) = sync::mpsc::channel();
    // let mut white = true;
    // let mut address = "localhost:6969";
    let mut white = false;
    let mut address = "localhost:6970";
    if let Some(arg) = args.get(1) {
        if arg.contains("black") {
            white = false;
            address = "localhost:6970";
        } else if arg.contains("white") {
            white = true;
            address = "localhost:6969";
        }
    }
    if let Some(arg) = args.get(2) {
        if arg.contains("alternative") || arg.contains("penis") {
            alternative_main(white, address);
            return;
        }
    }
    println!("parallelism: {:?}", thread::available_parallelism());
    let mut position_storer_ref;
    let mut hasher_ref;
    let mut time_left;
    let mut board;
    let mut board_clone;
    let hasher = Pin::new(Box::new(Hasher::new(69420)));
    let mut thread_count = 8;
    if let Ok(parallelism) = thread::available_parallelism() {
        thread_count = parallelism.into();
    }
    let mut prev_eval = 0;
    let mut socket = net::TcpStream::connect(address).expect("blemba nedai, https://www.isitdownrightnow.com/localhost.html");
    if let Err(e) = socket.set_nodelay(true) {
        println!("Nepavyko ijungti no delay communication su serveriu: {}", e);
    }
    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
        println!("Nepavyko pakeisti read timeout i 0.1 sekundes: {}", e);
    } else {
        println!("Read timeout yra 0.1 sekundes");
    }
    loop {
        if let Err(err) = socket.read_to_string(&mut string) {
            // nesvarbu
            // println!("nezinau ar svarbu, bet: {}", err);
        }
        if string.len() == 0 {
            continue;
        } else {
            println!("string: '{}'", string);
        }
        time_left = string
            .split(' ')
            .rev()
            .nth(if white {1} else {0})
            .unwrap()
            .parse::<u64>()
            .unwrap();
        board = Board::from_fen(&string);
        // couldnt be bothered to rework how my pointers and references worked
        board_clone = board.clone();
        position_storer.add_second_position(hasher.get_zobrist_hash(&board_clone));
        // nekartos tos pacios pozicijos 2 kart nes manys kad draw
        // galimai pades wacky mates darant
        if board.inside_naked_king_endgame() {
            position_storer.accent_repetitions = true;
        }
        println!("time_left: {}", time_left);
        println!("computing position, fen: '{}'", string);
        sender_clone = sender.clone();
        unsafe {
            position_storer_ref = PositionStorerRef::new(&mut *position_storer);
            hasher_ref = HasherRef::new(&*hasher);
            thread::spawn(move || {
                mp_compute_wrapper(board, position_storer_ref, hasher_ref, prev_eval, thread_count, time_left, min(10_000, time_left / 20), sender_clone);
            });
        };
        // let (eval, pos_move) = mp_compute_wrapper(board.clone(), &hasher, prev_eval, thread_count, time_left / 20);
        if let Ok((eval, choice)) = receiver.recv() {
            if let Some(actual_move) = choice {
                socket.write(actual_move.to_string().as_bytes()).expect("blemba nedai");
                println!("eval: {}; choice: {}", eval, actual_move.to_string());
                prev_eval = eval;
                board_clone.make_move(&actual_move);
                position_storer.add_first_position(hasher.get_zobrist_hash(&board_clone));
            } else {
                println!("eval: {}; choice: None", eval);
            }
        } else {
            println!("Something is sussy");
        }
        string.clear();
    }
    //
    // board.display_board();
    // if let Some(m) = top_level_search(&mut board, 5, 69420) {
    //     socket.write(m.to_string().as_bytes()).expect("blemba nedai");
    // } else {
    //     panic!("Pasiduodu");
    // }
    // // tests::main();
}

// fn main() {
//     tests::main();
// }


// fn mainer() {
//     // every vector needs to be initialized to a normal length
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
//     // if let Err(e) = socket.set_nodelay(true) {
//     //     println!("Nepavyko ijungti no delay communication su serveriu: {}", e);
//     // }
//     // let mut board = Board::from_starting_position();
//     // let mut board = Board::from_fen("r1bqkb1r/ppp2ppp/2n5/3np1N1/2B5/8/PPPP1PPP/RNBQK2R w KQkq - 0 6");
//     // let mut board = Board::from_fen("r1bqkb1r/ppp2p1p/2n3p1/3np1NQ/2B5/8/PPPP1PPP/RNB1K2R w KQkq - 0 7");
//     // let mut board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4");
//     // let mut board = Board::from_fen("r1bq1rk1/1pp2ppp/2p2n2/p1b5/4P3/2NB4/PPPP1PPP/R1BQ1RK1 w - - 0 8");
//     let mut board = Board::from_fen("r2q2nr/1pp1k1pp/2n2p2/pB1p4/1b1P2b1/2N1PN2/PPPB1PPP/R2QK2R w KQ - 3 9");
//     let hasher = Pin::new(Box::new(Hasher::new(69420)));
//     let mut search_args = SearchArgs::new(100, 20, &hasher);
//     let (sender, receiver) = sync::mpsc::channel();
//     let (sender2, receiver2) = sync::mpsc::channel();
//     // let mut vector = Vec::with_capacity(200);
//     let mut prev_eval = 0;
//     let thread_count = 8;
//     let mut thing;
//     let time_left = 100_000;
//     let dedicated_time = 20_000;
//     // unsafe {
//     //     thing = HasherRef::new(&*hasher);
//     //     thread::spawn(move || {
//     //         mp_compute_wrapper(board, thing, prev_eval, thread_count, dedicated_time, sender2);
//     //     });
//     // };
//     // let (eval, pos_move) = mp_compute_wrapper(board.clone(), &hasher, prev_eval, thread_count, time_left / 20);
//     if let Ok((eval, choice)) = receiver2.recv() {
//         if let Some(actual_move) = choice {
//             println!("eval: {}; choice: {}", eval, actual_move.to_string());
//         } else {
//             println!("eval: {}; choice: None", eval);
//         }
//     } else {
//         println!("Something is sussy");
//     }
//     println!("Time Out");
//     // panic!("unimportant");
//     thread::sleep(Duration::from_secs(10));
//     println!("Time In");
//     println!("Synchronous now:");
//     unsafe {
//         thing = HasherRef::new(&hasher);
//     }
//     board = Board::from_fen("r2q2nr/1pp1k1pp/2n2p2/pB1p4/1b1P2b1/2N1PN2/PPPB1PPP/R2QK2R w KQ - 3 9");
//     thread::spawn(move || {
//         compute(board, thing, prev_eval, dedicated_time, sender);
//     });
//     if let Ok((eval, choice)) = receiver.recv() {
//         if let Some(actual_move) = choice {
//             println!("eval: {}; choice: {}", eval, actual_move.to_string());
//         } else {
//             println!("eval: {}; choice: None", eval);
//         }
//     } else {
//         println!("Something is sussy");
//     }
// }

