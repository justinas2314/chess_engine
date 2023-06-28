use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::utils::{Board, generate_into, HashMapReuser, Move, VectorReuser, Visibility};
use crate::eval::*;
use crate::evaluation_function::*;
use crate::hashing::Hasher;

pub fn find_buggy_visibility(title: &'static str, subtitle: &str, board: &Board) {
    let mut flag = false;
    for x in 0..8 {
        for y in 0..8 {
            for (nx, ny) in board.visibility.map[x][y].total[0].iter() {
                if board.board[*nx][*ny].is_empty(){
                    println!("1: ({},{}) -> ({},{})", nx, ny, x, y);
                    flag = true;
                }
                if !board.visibility.from_to[*nx][*ny][x][y] {
                    println!("A: ({},{}) -> ({},{})", nx, ny, x, y);
                    flag = true;
                }
            }
            for (nx, ny) in board.visibility.map[x][y].total[1].iter() {
                if board.board[*nx][*ny].is_empty() {
                    println!("2: ({},{}) -> ({},{})", nx, ny, x, y);
                    flag = true;
                }
                if !board.visibility.from_to[*nx][*ny][x][y] {
                    println!("B: ({},{}) -> ({},{})", nx, ny, x, y);
                    flag = true;
                }
            }
            if board.board[x][y].is_empty() && board.visibility.pieces[x][y].len() > 0 {
                println!("({},{}) is not empty", x, y);
                flag = true;
            }
        }
    }
    if flag {
        println!("{}\n{}", title, subtitle);
        board.display_board();
        println!("{}", board.to_fen());
        board.display_debug_info();
        board.visibility.debug_info();
        panic!();
    }
}

fn duplicates(coords: &Vec<(usize, usize)>) -> bool {
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            if coords[i] == coords[j] {
                return true;
            }
        }
    }
    false
}

pub fn find_buggy_visibility_other(title: &'static str, subtitle: &str, visibility: &Visibility) {
    let mut flag = false;
    for x in 0..8 {
        for y in 0..8 {
            if duplicates(&visibility.pieces[x][y]) {
                println!("CODE 0: ({}, {}): {:?}", x, y, visibility.pieces[x][y]);
                flag = true;
            }
            if duplicates(&visibility.map[x][y].total[0]) {
                println!("CODE 1: ({}, {}): {:?}", x, y, visibility.map[x][y].total[0]);
                flag = true;
            }
            if duplicates(&visibility.map[x][y].total[1]) {
                println!("CODE 2: ({}, {}): {:?}", x, y, visibility.map[x][y].total[1]);
                flag = true;
            }
        }
    }
    if flag {
        println!("{}\n{}", title, subtitle);
        println!("{:?}", visibility);
        panic!();
    }
}

impl Board {
    pub fn debug_compute_evaluation(&self, hasher: &Hasher, pawn_structure_cache: &mut HashMap<u64, i64>) -> i64 {
        let mut piece;
        let mut output;
        let mut eyed_values = 0;
        let mut weighted_activity = 0;
        // white pieces attacking black king
        let mut white_attackers = 0;
        // black pieces attacking white king
        let mut black_attackers = 0;
        for i in KING_FEDENCE_NOZE[self.kings[1][0]][self.kings[1][1]].iter() {
            if let Some((x, y)) = *i {
                if self.visibility.map[x][y].total[0].len() > self.visibility.map[x][y].total[1].len() {
                    for (cx, cy) in self.visibility.map[x][y].total[0].iter() {
                        white_attackers += KING_ATTACK_VALUES[self.board[*cx][*cy] as usize]
                            * (self.visibility.map[x][y].total[0].len() - self.visibility.map[x][y].total[1].len());
                    }
                }
            } else {
                break;
            }
        }
        for i in KING_FEDENCE_NOZE[self.kings[0][0]][self.kings[0][1]].iter() {
            if let Some((x, y)) = *i {
                if self.visibility.map[x][y].total[1].len() > self.visibility.map[x][y].total[0].len() {
                    for (cx, cy) in self.visibility.map[x][y].total[1].iter() {
                        black_attackers += KING_ATTACK_VALUES[self.board[*cx][*cy] as usize]
                            * (self.visibility.map[x][y].total[1].len() - self.visibility.map[x][y].total[0].len());
                    }
                }
            } else {
                break;
            }
        }
        output = MATERIAL_WEIGHT * self.evaluation.material;
        println!("FIRST: += {}", MATERIAL_WEIGHT * self.evaluation.material);
        output += POSITIONING_WEIGHT * self.evaluation.positioning;
        println!("SECOND: += {}", POSITIONING_WEIGHT * self.evaluation.positioning);
        output += SAFETY_WEIGHT * (KING_SAFETY_VALUES[white_attackers] - KING_SAFETY_VALUES[black_attackers]);
        println!("THIRD: += {}", SAFETY_WEIGHT * (KING_SAFETY_VALUES[white_attackers] - KING_SAFETY_VALUES[black_attackers]));
        for x in 0..8 {
            for y in 0..8 {
                piece = self.board[x][y] as usize;
                weighted_activity += SQUARE_IMPORTANCE[x][y] *
                    (self.visibility.map[x][y].total[0].len() as i64 - self.visibility.map[x][y].total[1].len() as i64);
                if piece == 0 {
                    continue;
                }
                for (px, py) in self.visibility.map[x][y].total[0].iter() {
                    eyed_values += EYEING_VALUES[self.board[*px][*py] as usize][piece];
                }
                for (px, py) in self.visibility.map[x][y].total[1].iter() {
                    eyed_values += EYEING_VALUES[self.board[*px][*py] as usize][piece];
                }
            }
        }
        let pawn_hash = hasher.get_pawn_zobrist(self);
        if let Some(eval) = pawn_structure_cache.get(&pawn_hash) {
            output += eval;
        } else {
            let eval = PAWN_STRUCTURE_WEIGHT * self.eval_pawns();
            pawn_structure_cache.insert(pawn_hash, eval);
            output += eval;
        }
        output += eyed_values * EYE_WEIGHT;
        println!("FOURTH: += {}", eyed_values * EYE_WEIGHT);
        println!("FIFTH: += {}", weighted_activity * ACTIVITY_WEIGHT);
        output + weighted_activity * ACTIVITY_WEIGHT
    }
    pub fn start_perf(&mut self, depth: usize) -> usize {
        let mut local;
        let mut total = 0;
        let mut reuser = VectorReuser::with_capacity(1000);
        let mut moves = Vec::new();
        if depth == 0 {
            return 1;
        }
        generate_into(self, &mut moves);
        moves.sort_by(|a, b| {
            let s1 = a.to_string();
            let s2 = b.to_string();
            for i in [0, 2, 1, 3] {
                if s1[i..i + 1] != s2[i..i + 1] {
                    return s1[i..i + 1].cmp(&s2[i..i + 1]);
                }
            }
            Ordering::Equal
        });
        unsafe {
            for i in moves {
                self.make_move(&i);
                if self.hung_king() {
                    self.undo_move(&i);
                    continue;
                }
                local = self.perf(&mut reuser, depth - 1, 0);
                self.undo_move(&i);
                total += local;
                println!("{}: {} ({:?})", i.to_string(), local, i);
            }
        }
        total
    }
    pub unsafe fn perf(&mut self, reuser: &mut VectorReuser<Move>, depth: usize, index: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let pointer = reuser.get_vector_pointer(index);
        (*pointer).clear();
        generate_into(self, &mut(*pointer));
        let mut total = 0;
        // for i in (*pointer).iter().filter(|x| !x.is_en_passant()) {
        for i in (*pointer).iter() {
            // print_path(path, index);
            // self.display_board();
            self.make_move(i);
            if self.hung_king() {
                self.undo_move(i);
                continue;
            }
            total += self.perf(reuser, depth - 1, index + 1);
            self.undo_move(i);
        }
        total
    }
}

fn print_path(path: &mut [Option<Move>; 1000], index: usize) {
    for i in 0..=index {
        print!("{:?} -> ", path[i].unwrap());
    }
    println!();
}

impl Move {
    pub fn is_en_passant(&self) -> bool {
        match self {
            Move::EnPassant(..) => true,
            _ => false
        }
    }
}

impl Visibility {
    pub fn debug_info(&self) {
        let mut repr;
        println!("WHAT CAN ATTACK WHAT");
        for x in 0..8 {
            for y in 0..8 {
                repr = BoardRepr::new();
                repr.really_mark(x, y);
                for (cx, cy) in self.pieces[x][y].iter() {
                    repr.mark(*cx, *cy);
                }
                println!("0,{},{}", x, y);
                repr.display();
            }
        }
        println!("WHAT CAN BE ATTACKED BY WHITE");
        for x in 0..8 {
            for y in 0..8 {
                repr = BoardRepr::new();
                repr.really_mark(x, y);
                for (cx, cy) in self.map[x][y].total[0].iter() {
                    repr.mark(*cx, *cy);
                }
                println!("1,{},{}", x, y);
                repr.display();
            }
        }
        println!("WHAT CAN BE ATTACKED BY BLACK");
        for x in 0..8 {
            for y in 0..8 {
                repr = BoardRepr::new();
                repr.really_mark(x, y);
                for (cx, cy) in self.map[x][y].total[1].iter() {
                    repr.mark(*cx, *cy);
                }
                println!("2,{},{}", x, y);
                repr.display();
            }
        }
    }
}

pub struct BoardRepr {
    inner: [[usize; 8]; 8]
}

impl BoardRepr {
    pub fn new() -> BoardRepr {
        BoardRepr {
            inner: [[0; 8]; 8]
        }
    }
    pub fn mark(&mut self, x: usize, y: usize) {
        self.inner[x][y] += 1;
    }
    pub fn really_mark(&mut self, x: usize, y: usize) {
        self.inner[x][y] = 100;
    }
    pub fn display(&self) {
        let mut output = String::with_capacity(100);
        for i in 0..8 {
            for j in 0..8 {
                if self.inner[i][j] < 100 {
                    output.push_str(&self.inner[i][j].to_string());
                } else if self.inner[i][j] == 100 {
                    output.push('X');
                } else {
                    output.push('?');
                }
            }
            output.push('\n');
        }
        println!("{}", output);
    }
}