use crate::{MIN_DEPTH, MIN_DEPTH_TIME_LIMIT};
use crate::MAX_DEPTH;
use crate::eval::{DISAPPOINTMENT_VALUES, generate_killer_moves, get_obvious_move, get_previous_best_move, HasherRef, HOPE_VALUES, PositionStorerRef};
use crate::hashing::Hasher;
use crate::move_gen::generate_defenders_into;
use crate::utils::{Board, Visibility, SquareVisibility, Evaluation, VectorReuser, Move, HashMapReuser, BestMoves, generate_into, generate_captures_into, generate_check_evasions_into, generate_checks_into, get_capture, Square, generate_check_counters_into};

use std::cmp::{max, min};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::{sync, thread};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::Sender;
use std::thread::{JoinHandle, Thread};
use std::time::Duration;
use std::pin::Pin;
use rand::RngCore;

// jeigu naudociau derive negauciau privilegijos meditaciniam klavisu spaudinejimui
impl Clone for Evaluation {
    fn clone(&self) -> Evaluation {
        Evaluation {
            total_material: self.total_material,
            eyed_values: self.eyed_values,
            material: self.material,
            positioning: self.positioning
        }
    }
}

impl Clone for SquareVisibility {
    fn clone(&self) -> SquareVisibility {
        let mut output = SquareVisibility::new();
        for i in self.total[0].iter() {
            output.total[0].push(*i);
        }
        for i in self.total[1].iter() {
            output.total[1].push(*i);
        }
        output
    }
}

impl Clone for Visibility {
    fn clone(&self) -> Visibility {
        let mut output = Visibility::initialize();
        for fx in 0..8 {
            for fy in 0..8 {
                for tx in 0..8 {
                    for ty in 0..8 {
                        output.from_to[fx][fy][tx][ty] = self.from_to[fx][fy][tx][ty];
                    }
                }
            }
        }
        for x in 0..8 {
            for y in 0..8 {
                output.map[x][y] = self.map[x][y].clone();
                for i in self.pieces[x][y].iter() {
                    output.pieces[x][y].push(*i);
                }
            }
        }
        output
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        // let mut inner_board = [[Square::E; 8]; 8];
        // let mut en_passant_array = [[0; 8]; 8];
        // for x in 0..8 {
        //     for y in 0..8 {
        //         inner_board[x][y] = self.board[x][y];
        //         en_passant_array[x][y] = self.en_passant_array[x][y];
        //     }
        // }
        // let mut kings = [[0, 0], [0, 0]];
        // kings[0][0] = self.kings[0][0];
        // kings[0][1] = self.kings[0][1];
        // kings[1][0] = self.kings[1][0];
        // kings[1][1] = self.kings[1][1];
        // let mut king_moved = [0, 0];
        // king_moved[0] = self.king_moved[0];
        // king_moved[1] = self.king_moved[1];
        // let mut rook_moved = [0, 0, 0, 0];
        // rook_moved[0] = self.rook_moved[0];
        // rook_moved[1] = self.rook_moved[1];
        // rook_moved[2] = self.rook_moved[2];
        // rook_moved[3] = self.rook_moved[3];
        // let mut en_passant_hash_index = [8; 1000];
        // for i in 0..1000 {
        //     en_passant_hash_index[i] = self.en_passant_hash_index[i];
        // }
        Board {
            board: self.board.clone(),
            kings: self.kings.clone(),
            king_moved: self.king_moved.clone(),
            rook_moved: self.rook_moved.clone(),
            visibility: self.visibility.clone(),
            en_passant_array: self.en_passant_array.clone(),
            en_passant_hash_index: self.en_passant_hash_index.clone(),
            white_to_move: self.white_to_move,
            // pretty sure i never use this
            halfmove_clock: self.halfmove_clock,
            clock: self.clock,
            evaluation: self.evaluation.clone()
        }
    }
}

struct LiveAlphaBeta {
    // inner: AtomicI64,
    kill_yourself: Mutex<[bool; 2]>,
}

impl LiveAlphaBeta {
    pub fn new() -> LiveAlphaBeta {
        LiveAlphaBeta {
            // inner: AtomicI64::new(initial_alpha),
            kill_yourself: Mutex::new([false, false]),
        }
    }
    // pub fn get_value(&self) -> i64 {
    //     self.inner.load(Ordering::SeqCst)
    // }
    // pub fn get_alpha(&self, index: usize, alpha: i64) -> i64 {
    //     match index {
    //         0 => {
    //             let other = self.inner.load(Ordering::SeqCst);
    //             max(alpha, other)
    //         },
    //         _ => alpha
    //     }
    // }
    // pub fn set_value(&mut self, value: i64) {
    //     self.inner.swap(value, Ordering::SeqCst);
    // }
    // pub fn increase_alpha(&mut self, value: i64) {
    //     // check again to make sure im not trolled by a data race
    //     self.inner.fetch_max(value, Ordering::SeqCst);
    //     if let Ok(mut guard) = self.inner.write() {
    //         if value > *guard {
    //             *guard = value;
    //         }
    //     }
    // }
    pub fn commit_suicide(&mut self) {
        if let Ok(mut guard) = self.kill_yourself.lock() {
            guard[0] = true;
        } else {
            panic!("Something went wrong");
        }
    }
    pub fn commit_ultra_suicide(&mut self) {
        if let Ok(mut guard) = self.kill_yourself.lock() {
            guard[1] = true;
        } else {
            panic!("Something went wrong");
        }
    }
    pub fn resurrection(&mut self) {
        if let Ok(mut guard) = self.kill_yourself.lock() {
            guard[0] = false;
        } else {
            panic!("Something went wrong");
        }
    }
    pub fn contact_fuehrerbunker(&mut self) -> bool {
    // pub fn contact_fuehrerbunker(&mut self) {
        let bools = self.kill_yourself.get_mut().unwrap();
        // if bools[0] || bools[1] {
        //     panic!("controlled panic");
        // }
        return bools[0] || bools[1];
    }
    pub fn contact_oberfuehrerbunker(&mut self) -> bool {
        self.kill_yourself.get_mut().unwrap()[1]
    }
    pub fn take_order_from_fuehrer(&mut self) {
        let bools = self.kill_yourself.get_mut().unwrap();
        if bools[0] || bools[1] {
            panic!("controlled panic, mein fuehrer");
        }
    }
    pub fn take_order_from_oberfuehrer(&mut self) {
        if self.kill_yourself.get_mut().unwrap()[1] {
            panic!("controlled panic, mein oberfuehrer");
        }
    }
}

struct MPReuser<T> {
    inner: Vec<VectorReuser<T>>,
}

impl<T> MPReuser<T> {
    pub fn new(expected_thread_count: usize, expected_max_depth: usize, capacity: usize) -> MPReuser<T> {
        let mut inner = Vec::with_capacity(expected_thread_count);
        for _ in 0..expected_thread_count {
            inner.push(VectorReuser::with_capacity2d(expected_max_depth, capacity));
        }
        MPReuser {
            inner,
        }
    }
    pub fn get_reuser_pointer(&mut self, thread_index: usize) -> *mut VectorReuser<T> {
        &mut self.inner[thread_index]
    }
}

struct MPSearchArgsOwner<'a> {
    reusers: MPReuser<Move>,
    cache: Mutex<HashMapReuser<u64, i64>>,
    hasher: &'a Hasher,
    fuck_moves: Vec<Vec<BestMoves>>,
    marry_moves: Vec<[i64; 896]>,
    kill_moves: Mutex<[usize; 896]>,
    pawn_structure_cache: Mutex<HashMap<u64, i64>>,
    previous_best_moves: Mutex<HashMapReuser<u64, usize>>,
    // not indexed by colour but by index % 2
    live_alphabeta: Pin<Box<LiveAlphaBeta>>
}

impl<'a> MPSearchArgsOwner<'a> {
    pub fn new(hasher: &Hasher,
               threads: usize,
               max_move_depth: usize,
               max_eval_depth: usize) -> MPSearchArgsOwner {
        let mut fuck_moves = Vec::with_capacity(threads);
        let mut marry_moves = Vec::with_capacity(threads);
        for _ in 0..threads {
            fuck_moves.push([(); 100].iter().map(|_| BestMoves::new()).collect());
            marry_moves.push([0; 896]);
        }
        MPSearchArgsOwner {
            reusers: MPReuser::new(threads, max_move_depth, 500),
            cache: Mutex::new(HashMapReuser::with_capacity2d(max_eval_depth, 1_000_000)),
            hasher,
            fuck_moves,
            marry_moves,
            kill_moves: Mutex::new([4096; 896]),
            pawn_structure_cache: Mutex::new(HashMap::with_capacity(100_000)),
            previous_best_moves: Mutex::new(HashMapReuser::with_capacity2d(2, 10_000_000)),
            live_alphabeta: Pin::new(Box::new(LiveAlphaBeta::new()))
        }
    }
    // creates child
    pub fn sex(&mut self, index: usize) -> MPSearchArgs {
        MPSearchArgs {
            reuser: self.reusers.get_reuser_pointer(index),
            cache: &mut self.cache,
            hasher: self.hasher,
            fuck_moves: &mut self.fuck_moves[index],
            marry_moves: &mut self.marry_moves[index],
            kill_moves: &mut self.kill_moves,
            pawn_structure_cache: &mut self.pawn_structure_cache,
            previous_best_moves: &mut self.previous_best_moves,
            live_alphabeta: &mut *self.live_alphabeta,
        }
    }
    pub unsafe fn unprotected_sex(owner: *mut MPSearchArgsOwner, index: usize) -> MPSearchArgs {
        MPSearchArgs {
            reuser: (*owner).reusers.get_reuser_pointer(index),
            cache: &mut (*owner).cache,
            hasher: (*owner).hasher as *const _,
            fuck_moves: &mut (*owner).fuck_moves[index],
            marry_moves: &mut (*owner).marry_moves[index],
            kill_moves: &mut (*owner).kill_moves,
            pawn_structure_cache: &mut (*owner).pawn_structure_cache,
            previous_best_moves: &mut (*owner).previous_best_moves,
            live_alphabeta: &mut *(*owner).live_alphabeta,
        }
    }
    pub fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
        // for i in self.fuck_moves.iter_mut() {
        //     for j in i.iter_mut() {
        //         j.moves[0] = 4096;
        //         j.moves[1] = 4096;
        //         j.moves[2] = 4096;
        //     }
        // }
    }
}

unsafe impl Send for MPSearchArgs {}

// yolo lol
struct MPSearchArgs {
    reuser: *mut VectorReuser<Move>,
    cache: *mut Mutex<HashMapReuser<u64, i64>>,
    hasher: *const Hasher,
    fuck_moves: *mut Vec<BestMoves>,
    marry_moves: *mut [i64; 896],
    kill_moves: *mut Mutex<[usize; 896]>,
    pawn_structure_cache: *mut Mutex<HashMap<u64, i64>>,
    previous_best_moves: *mut Mutex<HashMapReuser<u64, usize>>,
    // not indexed by colour but by index % 2
    live_alphabeta: *mut LiveAlphaBeta,
}

impl MPSearchArgs {
    pub unsafe fn clear(&mut self) {
        (&mut *self.marry_moves).fill(0);
    }
}


pub unsafe fn mp_compute_wrapper(board: Board,
                                 position_storer: PositionStorerRef,
                                 hasher: HasherRef,
                                 prev_eval: i64,
                                 thread_count: usize,
                                 time_left: u64,
                                 dedicated_time_ms: u64,
                                 output_channel: Sender<(i64, Option<Move>)>) {
    let mut output = (prev_eval, None);
    let mut arg_owner = MPSearchArgsOwner::new(
        // safety is our number one priority
        &*hasher.0,
        thread_count,
        100,
        MAX_DEPTH
    );
    let genocide_pointer: *mut LiveAlphaBeta = &mut *arg_owner.live_alphabeta;
    let (sender, receiver) = sync::mpsc::channel();
    let (done_sender, done_receiver) = sync::mpsc::channel();
    let (deepened_sender, deepened_receiver) = sync::mpsc::channel();
    let done_clone = done_sender.clone();
    let child = thread::spawn(move || {
        mp_compute(position_storer, deepened_sender, sender, arg_owner, board, prev_eval, thread_count);
        done_clone.send(());
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(dedicated_time_ms));
        done_sender.send(());
    });
    if time_left > MIN_DEPTH_TIME_LIMIT {
        // wait until minimum depth is reached
        if let Ok(()) = deepened_receiver.recv() {}
    }
    // wait until either dedicated time runs out or position gets analysed enough
    if let Ok(()) = done_receiver.recv() {}
    while let Ok(choice) = receiver.try_recv() {
        println!("choices: {:?}", choice);
        output = choice;
    }
    output_channel.send(output).unwrap();
    (*genocide_pointer).commit_ultra_suicide();
    child.join().unwrap();
    println!("THREAD JOINED");
}

fn mp_compute(position_storer: PositionStorerRef,
              reached_depth: Sender<()>,
              output_channel: Sender<(i64, Option<Move>)>,
              mut arg_owner: MPSearchArgsOwner,
              mut board: Board,
              mut prev_eval: i64,
              thread_count: usize) {
    let mut chosen_move;
    let mut output_channel_clone;
    let mut vector = Vec::with_capacity(200);
    for depth in 1..MIN_DEPTH {
        if arg_owner.live_alphabeta.contact_oberfuehrerbunker() {
            return;
        }
        println!("depth: {}", depth);
        output_channel_clone = output_channel.clone();
        (prev_eval, chosen_move) = unsafe {
            mp_deepen_search(position_storer, output_channel_clone, &mut arg_owner, &mut vector, &mut board, prev_eval, thread_count, depth)
        };
        if chosen_move.is_none() || prev_eval > 100_000_000 {
            reached_depth.send(());
            return;
        }
    }
    reached_depth.send(());
    for depth in MIN_DEPTH..MAX_DEPTH {
        if arg_owner.live_alphabeta.contact_oberfuehrerbunker() {
            return;
        }
        println!("depth: {}", depth);
        output_channel_clone = output_channel.clone();
        (prev_eval, chosen_move) = unsafe {
            mp_deepen_search(position_storer, output_channel_clone, &mut arg_owner, &mut vector, &mut board, prev_eval, thread_count, depth)
        };
        if chosen_move.is_none() || prev_eval > 100_000_000 {
            return;
        }
    }
}

unsafe fn mp_deepen_search(position_storer: PositionStorerRef,
                           output_channel: Sender<(i64, Option<Move>)>,
                           arg_owner: &mut MPSearchArgsOwner,
                           vector: &mut Vec<Move>,
                           board: &mut Board,
                           previous_eval: i64,
                           thread_count: usize,
                           depth: usize) -> (i64, Option<Move>) {
    // at some point this became semi-ironic but i don't remember when
    let mut alpha;
    let mut iterator_index;
    let mut move_copy;
    let mut board_copy;
    let mut sender_copy;
    let mut mp_search_args;
    let mut best_move = None;
    let mut min_index = 0;
    let mut max_index = 1;
    let mut shitty_moves;
    let mut active_thread_count;
    let mut expected_floor = previous_eval - HOPE_VALUES[min_index];
    let mut expected_ceiling = previous_eval + HOPE_VALUES[max_index];
    let (sender, receiver) = sync::mpsc::channel();
    let mut threads = Vec::with_capacity(MAX_DEPTH);
    vector.clear();
    generate_into(board, vector);
    vector.sort_unstable_by_key(|a| -a.guesstimate_delta());
    arg_owner.clear();
    (*position_storer.0).update_with_draws(arg_owner.cache.get_mut().unwrap());
    // depth -= 1;
    'outer: loop {
        // arg_owner.live_alphabeta.take_order_from_oberfuehrer();
        if arg_owner.live_alphabeta.contact_oberfuehrerbunker() {
            // does not do anything I do not send this to the output channel
            return (previous_eval, best_move);
        }
        while let Ok(_) = receiver.try_recv() {}
        // arg_owner.cache.get_mut().unwrap().hashmaps.clear();
        // in theory this might not be 0 so im not risking wacky bugs
        shitty_moves = 0;
        iterator_index = 0;
        active_thread_count = 0;
        alpha = expected_floor;
        // arg_owner.live_alphabeta.set_value(alpha);
        for thread_index in 0..thread_count {
            if let Some(current_move) = vector.get(iterator_index) {
                iterator_index += 1;
                sender_copy = sender.clone();
                board_copy = board.clone();
                move_copy = current_move.clone();
                mp_search_args = MPSearchArgsOwner::unprotected_sex(arg_owner, thread_index);
                mp_search_args.clear();
                threads.push(Some(thread::spawn(move || {
                    sp_search_wrapper(mp_search_args,
                                      board_copy,
                                      depth,
                                      -expected_ceiling,
                                      -alpha,
                                      move_copy,
                                      thread_index,
                                      sender_copy);
                })));
                // threads.push(Some(thread::Builder::new().name(format!("thread index: {}", thread_index)).spawn(move || {
                //         sp_search_wrapper(mp_search_args,
                //                           board_copy,
                //                           depth,
                //                           -expected_ceiling,
                //                           -alpha,
                //                           move_copy,
                //                           thread_index,
                //                           sender_copy);
                // }).unwrap()));
                active_thread_count += 1;
            } else {
                // sitas else break is principo parasytas
                // (db po vidurnakcio as isgeriau energetinio jauciu kaip trippinu)
                break;
            }
        }
        while active_thread_count > 0 {
            if arg_owner.live_alphabeta.contact_oberfuehrerbunker() {
                for i in threads.drain(0..) {
                    if let Some(handle) = i {
                        handle.join().unwrap();
                    }
                }
                // does not do anything I do not send this to the output channel
                return (alpha, best_move);
            }
            // move is a keyword in rust -> move = pos_delta
            if let Ok((eval, index, pos_delta)) = receiver.recv() {
                if arg_owner.live_alphabeta.contact_oberfuehrerbunker() {
                    for i in threads.drain(0..) {
                        if let Some(handle) = i {
                            handle.join().unwrap();
                        }
                    }
                    return (alpha, best_move)
                }
                // println!("{iterator_index} {depth}, ({alpha}, {expected_ceiling}) {eval}, {index}, {:?}", pos_delta);
                if eval > alpha {
                    if eval >= expected_ceiling {
                        // starts killing threads first
                        arg_owner.live_alphabeta.commit_suicide();
                        max_index += 1;
                        expected_ceiling = previous_eval + HOPE_VALUES[max_index];
                        // println!("FAILED HIGH");
                        for i in threads.drain(0..) {
                            if let Some(handle) = i {
                                handle.join().unwrap();
                            }
                        }
                        arg_owner.live_alphabeta.resurrection();
                        continue 'outer;
                    }
                    best_move = Some(pos_delta);
                    alpha = eval;
                    // arg_owner.live_alphabeta.increase_alpha(alpha);
                } else if eval == -1_000_000_000 {
                    shitty_moves += 1;
                } else if best_move.is_none() {
                    best_move = Some(pos_delta);
                }
                if let Some(pos_move) = vector.get(iterator_index) {
                    iterator_index += 1;
                    sender_copy = sender.clone();
                    board_copy = board.clone();
                    move_copy = pos_move.clone();
                    mp_search_args = MPSearchArgsOwner::unprotected_sex(arg_owner, index);
                    mp_search_args.clear();
                    if let Some(handle) = threads[index].take() {
                        handle.join().unwrap();
                    }
                    threads[index] = Some(thread::spawn(move || {
                        sp_search_wrapper(mp_search_args,
                                          board_copy,
                                          depth,
                                          -expected_ceiling,
                                          -alpha,
                                          move_copy,
                                          index,
                                          sender_copy);
                    }));
                    // threads[index] = Some(thread::Builder::new().name(format!("thread index: {}", index)).spawn(move || {
                    //         sp_search_wrapper(mp_search_args,
                    //                           board_copy,
                    //                           depth,
                    //                           -expected_ceiling,
                    //                           -alpha,
                    //                           move_copy,
                    //                           index,
                    //                           sender_copy);
                    // }).unwrap());
                } else {
                    active_thread_count -= 1;
                }
            }
        }
        if shitty_moves + 1 == vector.len() {
            output_channel.send((1_000_000_000, best_move));
            return (1_000_000_000, best_move)
        }
        if alpha <= expected_floor {
            arg_owner.live_alphabeta.commit_suicide();
            min_index += 1;
            expected_floor = previous_eval - HOPE_VALUES[min_index];
            // println!("FAILED LOW");
            for i in threads.drain(0..) {
                if let Some(handle) = i {
                    handle.join().unwrap();
                }
            }
            arg_owner.live_alphabeta.resurrection();
        } else {
            output_channel.send((alpha, best_move));
            for i in threads.drain(0..) {
                if let Some(handle) = i {
                    handle.join().unwrap();
                }
            }
            return (alpha, best_move);
        }
    }
}

unsafe fn sp_search_wrapper(mut mp_search_args: MPSearchArgs,
                            mut board: Board,
                            depth: usize,
                            alpha: i64,
                            beta: i64,
                            current_move: Move,
                            thread_index: usize,
                            sender: Sender<(i64, usize, Move)>) {
    let piece_hash = current_move.get_piece_hash(&board);
    board.make_move(&current_move);
    if let Ok(mut guard) = (*mp_search_args.cache).lock() {
        if let Some(eval) = guard.get(0, &(*mp_search_args.hasher).get_zobrist_hash(&board)) {
            sender.send((*eval, thread_index, current_move)).unwrap();
        }
    }
    if board.hung_king() {
        sender.send((-1_000_000_000, thread_index, current_move)).unwrap();
        return;
    }
    let res = sp_search(&mut mp_search_args, &mut board, depth, 1, alpha, beta, piece_hash);
    // no need to undo move since board is dropped
    sender.send((-res, thread_index, current_move)).unwrap();
}


unsafe fn sp_search(mp_search_args: &mut MPSearchArgs,
                    board: &mut Board,
                    depth: usize,
                    index: usize,
                    prev_alpha: i64,
                    beta: i64,
                    last_move: usize) -> i64 {
    let hash;
    // let mut move_hash;
    let mut temp_eval;
    let mut pos_move;
    let coord_hash;
    let vector;
    let mut move_piece_hash;
    let mut hung_count = 0;
    let mut chosen_move = 4096;
    let mut alpha = prev_alpha;
    // hash = board.get_hash();
    if (*mp_search_args.live_alphabeta).contact_fuehrerbunker() {
        return alpha;
    }
    // (*mp_search_args.live_alphabeta).take_order_from_fuehrer();
    hash = (*mp_search_args.hasher).get_zobrist_hash(&board);
    if let Ok(mut guard) = (*mp_search_args.cache).lock() {
        if let Some(x) = guard.get(index, &hash) {
            return *x;
        }
    }
    debug_assert!(alpha < beta);
    debug_assert!(board.evaluation.total_material >= 0, "{}", board.to_fen());
    match depth - index {
        0 => {
            temp_eval = sp_quiescence_search(mp_search_args.reuser, true, board, index, alpha, beta, mp_search_args.hasher, mp_search_args.pawn_structure_cache);
            if temp_eval > alpha {
                if temp_eval >= beta {
                    return beta;
                }
                if let Ok(mut guard) = (*mp_search_args.cache).lock() {
                    guard.insert(index, hash, temp_eval);
                }
                return temp_eval;
            }
            return alpha;
        },
        // sorry compiler please optimise this
        _ if board.in_check() => {
            vector = &mut(*(*mp_search_args.reuser).get_vector_pointer(index));
            vector.clear();
            generate_check_counters_into(board, vector);
            vector.sort_unstable_by_key(|a| a.to_u8());
            for i in vector.iter().rev() {
                board.make_move(i);
                if board.hung_king() {
                    hung_count += 1;
                    board.undo_move(i);
                    continue;
                }
                move_piece_hash = i.get_piece_hash(board);
                temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
                board.undo_move(i);
                if temp_eval > alpha {
                    if temp_eval >= beta {
                        return beta
                    }
                    alpha = temp_eval;
                }
            }
            if hung_count == vector.len() {
                alpha = -1_000_000_000 + index as i64;
            }
            if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
                guard.insert(index % 2, hash, chosen_move);
            }
            return alpha;
        },
        1 => {
            temp_eval = sp_quiescence_search(mp_search_args.reuser, true, board, index + 1, alpha, beta, mp_search_args.hasher, mp_search_args.pawn_structure_cache);
            if temp_eval + DISAPPOINTMENT_VALUES[0] < alpha {
                return temp_eval;
            }
        },
        2 => {
            temp_eval = sp_quiescence_search(mp_search_args.reuser, true, board, index + 2, alpha, beta, mp_search_args.hasher, mp_search_args.pawn_structure_cache);
            if temp_eval + DISAPPOINTMENT_VALUES[1] < alpha {
                return temp_eval;
            }
        },
        _ => ()
    }
    vector = &mut(*(*mp_search_args.reuser).get_vector_pointer(index));
    pos_move = if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
        get_previous_best_move(board, &mut *guard, index, &hash)
    } else {
        panic!("SOMETHING WENT WRONG");
    };
    if let Some(i) = pos_move {
        board.make_move(&i);
        if board.hung_king() {
            board.undo_move(&i);
        } else {
            move_piece_hash = i.get_piece_hash(board);
            temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
            board.undo_move(&i);
            if temp_eval > alpha {
                if temp_eval >= beta {
                    if !i.is_capture() {
                        (*mp_search_args.marry_moves)[move_piece_hash] += 2i64.pow((depth - index) as u32);
                    }
                    coord_hash = i.get_coord_hash();
                    if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
                        guard[last_move] = coord_hash;
                    }
                    return beta
                }
                alpha = temp_eval;
                chosen_move = i.get_coord_hash();
            }
        }
    }
    pos_move = if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
        get_obvious_move(board, &mut *guard, last_move)
    } else {
        panic!("SOMETHING WENT WRONG");
    };
    if let Some(i) = pos_move {
        board.make_move(&i);
        if board.hung_king() {
            board.undo_move(&i);
        } else {
            move_piece_hash = i.get_piece_hash(board);
            temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
            board.undo_move(&i);
            if temp_eval > alpha {
                if temp_eval >= beta {
                    if !i.is_capture() {
                        (*mp_search_args.marry_moves)[move_piece_hash] += 2i64.pow((depth - index) as u32);
                    }
                    coord_hash = i.get_coord_hash();
                    if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
                        guard[last_move] = coord_hash;
                    }
                    if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
                        guard.insert(index % 2, hash, coord_hash);
                    }
                    return beta
                }
                alpha = temp_eval;
                chosen_move = i.get_coord_hash();
            }
        }
    }
    vector.clear();
    generate_captures_into(board, vector);
    vector.sort_unstable_by_key(|a| -a.guesstimate_delta() - (*mp_search_args.marry_moves)[a.get_piece_hash(board)]);
    for i in vector.iter().rev() {
        // since i have this condition cannot assume that only non-captures will raise alpha
        // in the second for loop
        // if eval + i.get_capture_delta() < alpha || see_capture(i, &mut args.reuser, board, index + 1) < 0 {
        if sp_see_capture(i, mp_search_args.reuser, board, index + 1) < 0 {
            continue;
        }
        board.make_move(i);
        if board.hung_king() {
            board.undo_move(i);
            continue;
        }
        move_piece_hash = i.get_piece_hash(board);
        temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
        board.undo_move(i);
        if temp_eval > alpha {
            if temp_eval >= beta {
                coord_hash = i.get_coord_hash();
                if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
                    guard[last_move] = coord_hash;
                }
                if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
                    guard.insert(index % 2, hash, coord_hash);
                }
                return beta
            }
            alpha = temp_eval;
            chosen_move = i.get_coord_hash();
        }
    }
    vector.clear();
    generate_killer_moves(board, vector, index, &mut *mp_search_args.fuck_moves);
    for i in vector.iter() {
        board.make_move(i);
        if board.hung_king() {
            board.undo_move(i);
            continue;
        }
        move_piece_hash = i.get_piece_hash(board);
        temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
        board.undo_move(i);
        if temp_eval > alpha {
            if temp_eval >= beta {
                if !i.is_capture() {
                    (*mp_search_args.marry_moves)[move_piece_hash] += 2i64.pow((depth - index) as u32);
                }
                coord_hash = i.get_coord_hash();
                if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
                    guard[last_move] = coord_hash;
                }
                if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
                    guard.insert(index % 2, hash, coord_hash);
                }
                return beta
            }
            alpha = temp_eval;
            chosen_move = i.get_coord_hash();
        }
    }
    vector.clear();
    // generate the same moves again but i cant be bothered to change that
    generate_into(board, vector);
    vector.sort_unstable_by_key(|a| -a.guesstimate_delta() - (*mp_search_args.marry_moves)[a.get_piece_hash(board)]);
    for i in vector.iter().rev() {
        board.make_move(i);
        if board.hung_king() {
            board.undo_move(i);
            hung_count += 1;
            continue;
        }
        move_piece_hash = i.get_piece_hash(board);
        temp_eval = -sp_search(mp_search_args, board, depth, index + 1, -beta, -alpha, move_piece_hash);
        board.undo_move(i);
        if temp_eval > alpha {
            if temp_eval >= beta {
                coord_hash = i.get_coord_hash();
                if !i.is_capture() {
                    (*mp_search_args.marry_moves)[move_piece_hash] += 2i64.pow((depth - index) as u32);
                    (*mp_search_args.fuck_moves)[index].insert(coord_hash);
                }
                if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
                    guard[last_move] = coord_hash;
                }
                if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
                    guard.insert(index % 2, hash, coord_hash);
                }
                return beta
            }
            alpha = temp_eval;
            chosen_move = i.get_coord_hash();
        }
    }
    if vector.len() == hung_count {
        // the mate is stale, I know that im not in check
        alpha = 0;
    } else if let Ok(mut guard) = (*mp_search_args.kill_moves).lock() {
        guard[last_move] = chosen_move;
    }
    if let Ok(mut guard) = (*mp_search_args.previous_best_moves).lock() {
        guard.insert(index % 2, hash, chosen_move);
    }
    if alpha > prev_alpha {
        if alpha >= beta {
            return beta;
        }
        if let Ok(mut guard) = (*mp_search_args.cache).lock() {
            guard.insert(index, hash, alpha);
        }
    }
    alpha
}

// this misses tactics where you can check twice and then win a piece
// and moves with temporary material sacrifices
// but should be good enough + this makes it faster
unsafe fn sp_quiescence_search(reuser: *mut VectorReuser<Move>,
                               check_check: bool,
                               board: &mut Board,
                               index: usize,
                               mut alpha: i64,
                               beta: i64,
                               hasher: *const Hasher,
                               pawn_structure_cache: *mut Mutex<HashMap<u64, i64>>) -> i64 {
    let mut temp_eval;
    let vector;
    if board.in_check() {
        // literally went insane bc of this
        // but the mate value from quiescence and the normal eval
        // are different (its a feature actually)
        vector = &mut(*(*reuser).get_vector_pointer(index));
        vector.clear();
        generate_check_counters_into(board, vector);
        vector.sort_unstable_by_key(|a| a.to_u8());
        for i in vector.iter().rev() {
            board.make_move(i);
            if board.hung_king() {
                board.undo_move(i);
                continue;
            }
            temp_eval = -sp_quiescence_search(reuser, check_check, board, index + 1, -beta, -alpha, hasher, pawn_structure_cache);
            board.undo_move(i);
            if temp_eval > alpha {
                if temp_eval >= beta {
                    return beta
                }
                alpha = temp_eval;
            }
        }
        // pretty sure if I just return alpha this score will propogate
        // up towards the outward most layer as equal to beta and will cause a crash
        return max(alpha, -100_000_000 + index as i64);
        // return alpha;
    }
    let eval = board.sp_evaluate(&*hasher, pawn_structure_cache);
    if eval > alpha {
        if eval >= beta {
            return beta;
        }
        alpha = eval;
    }
    vector = &mut(*(*reuser).get_vector_pointer(index));
    vector.clear();
    generate_captures_into(board, vector);
    vector.sort_unstable_by_key(|a| a.to_u8());
    for i in vector.iter().rev() {
        if eval + i.get_capture_delta() < alpha || sp_see_capture(i, reuser, board, index + 1) < 0 {
            continue
        }
        board.make_move(i);
        if board.hung_king() {
            board.undo_move(i);
            continue;
        }
        temp_eval = -sp_quiescence_search(reuser, true, board, index + 1, -beta, -alpha, hasher, pawn_structure_cache);
        board.undo_move(i);
        if temp_eval > alpha {
            if temp_eval >= beta {
                return beta
            }
            alpha = temp_eval;
        }
    }
    if check_check {
        vector.clear();
        generate_checks_into(board, vector);
        for i in vector.iter() {
            board.make_move(i);
            if board.hung_king() {
                board.undo_move(i);
                continue;
            }
            temp_eval = -sp_quiescence_search(reuser, false, board, index + 1, -beta, -alpha, hasher, pawn_structure_cache);
            board.undo_move(i);
            if temp_eval > alpha {
                if temp_eval >= beta {
                    return beta
                }
                alpha = temp_eval;
            }
        }
    }
    alpha
}

unsafe fn sp_see_capture(capture: &Move, reuser: *mut VectorReuser<Move>, board: &mut Board, index: usize) -> i64 {
    let mut output = 0;
    let (x, y) = capture.get_xy();
    if let Some(worth) = capture.get_captured_piece_worth() {
        board.make_move(&capture);
        output = worth - static_exchange_evaluation(reuser, board, index, x, y);
        board.undo_move(&capture);
    }
    output
}

unsafe fn static_exchange_evaluation(reuser: *mut VectorReuser<Move>, board: &mut Board, index: usize, x: usize, y: usize) -> i64 {
    let mut output = 0 ;
    let vector = &mut(*(*reuser).get_vector_pointer(index));
    let move_with_smallest_piece;
    vector.clear();
    generate_defenders_into(board, vector, x, y);
    if vector.len() == 0 {
        return output
    }
    move_with_smallest_piece = vector.iter().max_by_key(|a| a.to_u8()).unwrap().clone();
    if let Some(worth) = move_with_smallest_piece.get_captured_piece_worth() {
        board.make_move(&move_with_smallest_piece);
        output = max(0, worth - static_exchange_evaluation(reuser, board, index + 1, x, y));
        board.undo_move(&move_with_smallest_piece);
    }
    output
}
