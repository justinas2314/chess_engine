use crate::move_gen::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;


pub struct VectorReuser<T> {
    vectors: Vec<Vec<T>>
}

impl<T> VectorReuser<T> {
    pub fn new() -> VectorReuser<T> {
        VectorReuser {
            vectors: Vec::new()
        }
    }
    pub fn with_capacity(capacity: usize) -> VectorReuser<T> {
        VectorReuser {
            vectors: Vec::with_capacity(capacity)
        }
    }
    pub fn get_vector_pointer(&mut self, index: usize) -> *mut Vec<T> {
        if self.vectors.get(index).is_none() {
            self.vectors.push(Vec::new());
        }
        &mut self.vectors[index]
    }
}

pub struct HashMapReuser<K: Eq + Hash, V> {
    hashmaps: Vec<HashMap<K, V>>,
    capacity: usize
}

impl<K: Eq + Hash, V> HashMapReuser<K, V> {
    pub fn new() -> HashMapReuser<K, V> {
        HashMapReuser {
            hashmaps: Vec::new(),
            capacity: 100_000
        }
    }
    pub fn with_capacity(capacity: usize) -> HashMapReuser<K, V> {
        HashMapReuser {
            hashmaps: Vec::new(),
            capacity
        }
    }
    pub fn insert(&mut self, index: usize, key: K, value: V) {
        if self.hashmaps.get(index).is_none() {
            self.hashmaps.push(HashMap::with_capacity(self.capacity));
        }
        self.hashmaps[index].insert(key, value);
    }
    pub fn get(&mut self, index: usize, key: &K) -> Option<&V> {
        if index == self.hashmaps.len() {
            self.hashmaps.push(HashMap::with_capacity(self.capacity));
            self.hashmaps[index].get(key)
        } else {
            self.hashmaps[index].get(key)
        }
    }
}


#[derive(Hash, Debug, Copy, Clone)]
pub enum Square {
    E,
    P { white: bool },
    N { white: bool },
    B { white: bool },
    R { white: bool },
    Q { white: bool },
    K { white: bool },
}

impl Square {
    fn from_char(c: char) -> Square {
        match c {
            'P' => Square::P { white: true },
            'p' => Square::P { white: false },
            'N' => Square::N { white: true },
            'n' => Square::N { white: false },
            'B' => Square::B { white: true },
            'b' => Square::B { white: false },
            'R' => Square::R { white: true },
            'r' => Square::R { white: false },
            'Q' => Square::Q { white: true },
            'q' => Square::Q { white: false },
            'K' => Square::K { white: true },
            'k' => Square::K { white: false },
            _ => Square::E
        }
    }
    fn display_square(&self) -> char {
        match self {
            Square::E => '.',
            Square::P { white: true } => 'P',
            Square::P { white: false } => 'p',
            Square::N { white: true } => 'N',
            Square::N { white: false } => 'n',
            Square::B { white: true } => 'B',
            Square::B { white: false } => 'b',
            Square::R { white: true } => 'R',
            Square::R { white: false } => 'r',
            Square::Q { white: true } => 'Q',
            Square::Q { white: false } => 'q',
            Square::K { white: true } => 'K',
            Square::K { white: false } => 'k'
        }
    }
    pub fn worth(&self) -> isize {
        match self {
            Square::E => 0,
            Square::P{white: _} => 1,
            Square::N{white: _} => 3,
            Square::B{white: _} => 3,
            Square::R{white: _} => 5,
            Square::Q{white: _} => 9,
            Square::K{white: _} => 10_000
        }
    }
    pub fn bishopy_enemy(&self, white_move: bool) -> usize {
        match self {
            Square::E => 1,
            Square::B { white } => if *white != white_move {2} else {0},
            Square::Q { white } => if *white != white_move {2} else {0},
            _ => 0
        }
    }
    pub fn rooky_enemy(&self, white_move: bool) -> usize {
        match self {
            Square::E => 1,
            Square::R { white } => if *white != white_move {2} else {0},
            Square::Q { white } => if *white != white_move {2} else {0},
            _ => 0
        }
    }
    pub fn can_capture(&self, white_move: bool) -> bool {
        match self {
            Square::E => false,
            Square::P { white } => *white != white_move,
            Square::N { white } => *white != white_move,
            Square::B { white } => *white != white_move,
            Square::R { white } => *white != white_move,
            Square::Q { white } => *white != white_move,
            Square::K { white } => *white != white_move
        }
    }
    pub fn can_move_to(&self, white_move: bool) -> bool {
        match self {
            Square::E => true,
            Square::P { white } => *white != white_move,
            Square::N { white } => *white != white_move,
            Square::B { white } => *white != white_move,
            Square::R { white } => *white != white_move,
            Square::Q { white } => *white != white_move,
            Square::K { white } => *white != white_move
        }
    }
    pub fn can_move_to_and_through(&self, white_move: bool) -> (bool, bool) {
        match self {
            Square::E => (true, true),
            Square::P { white } => (*white != white_move, false),
            Square::N { white } => (*white != white_move, false),
            Square::B { white } => (*white != white_move, false),
            Square::R { white } => (*white != white_move, false),
            Square::Q { white } => (*white != white_move, false),
            Square::K { white } => (*white != white_move, false)
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Square::E => true,
            _ => false
        }
    }
    pub fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Square::E, Square::E) => Ordering::Equal,
            (Square::E, _) => Ordering::Less,
            (_, Square::E) => Ordering::Greater,
            (Square::P{white: _}, Square::P{white: _}) => Ordering::Equal,
            (Square::P{white: _}, _) => Ordering::Less,
            (_, Square::P{white: _}) => Ordering::Greater,
            (Square::N{white: _}, Square::B{white: _}) => Ordering::Equal,
            (Square::B{white: _}, Square::N{white: _}) => Ordering::Equal,
            (Square::N{white: _}, Square::N{white: _}) => Ordering::Equal,
            (Square::B{white: _}, Square::B{white: _}) => Ordering::Equal,
            (Square::N{white: _}, _) => Ordering::Less,
            (_, Square::N{white: _}) => Ordering::Greater,
            (Square::B{white: _}, _) => Ordering::Less,
            (_, Square::B{white: _}) => Ordering::Greater,
            (Square::R{white: _}, Square::R{white: _}) => Ordering::Equal,
            (Square::R{white: _}, _) => Ordering::Less,
            (_, Square::R{white: _}) => Ordering::Greater,
            (Square::Q{white: _}, Square::Q{white: _}) => Ordering::Equal,
            (Square::Q{white: _}, _) => Ordering::Less,
            (_, Square::Q{white: _}) => Ordering::Greater,
            (Square::K{white: _}, Square::K{white: _}) => Ordering::Equal
        }
    }
}

#[derive(Debug, Clone)]
pub enum Move {
    // what was captured and what it promoted to
    PawnPromotion(usize, usize, usize, usize, Square, Square),
    // used for sorting
    PawnCapture(usize, usize, usize, usize, Square),
    // where the pawn (doing the capturing) was and where it should be placed
    EnPassant(usize, usize, usize, usize, Square),
    // where the piece was, where it moved and what was on the square before
    KnightMove(usize, usize, usize, usize, Square),
    BishopMove(usize, usize, usize, usize, Square),
    RookMove(usize, usize, usize, usize, Square),
    QueenMove(usize, usize, usize, usize, Square),
    KingMove(usize, usize, usize, usize, Square),
    PawnPush(usize, usize, usize, usize),
    TwoSquarePawnMove(usize, usize, usize, usize),
    // where the king was and where he should be placed
    Castling(usize, usize, usize, usize)
}

impl Move {
    pub fn get_xy(&self) -> (usize, usize) {
        match self {
            Move::PawnPromotion(_, _, x, y, _, _) => (*x, *y),
            Move::PawnCapture(_, _, x, y, _) => (*x, *y),
            Move::EnPassant(_, _, x, y, _) => (*x, *y),
            Move::KnightMove(_, _, x, y, _) => (*x, *y),
            Move::BishopMove(_, _, x, y, _) => (*x, *y),
            Move::RookMove(_, _, x, y, _) => (*x, *y),
            Move::QueenMove(_, _, x, y, _) => (*x, *y),
            Move::KingMove(_, _, x, y, _) => (*x, *y),
            Move::PawnPush(_, _, x, y) => (*x, *y),
            Move::TwoSquarePawnMove(_, _, x, y) => (*x, *y),
            Move::Castling(_, _, x, y) => (*x, *y)
        }
    }
    pub fn get_captured_piece_worth(&self) -> Option<isize> {
        match self {
            Move::PawnPromotion(_, _, _, _, a, b) => Some(a.worth() + b.worth()),
            Move::PawnCapture(_, _, _, _, a) => Some(a.worth()),
            Move::EnPassant(_, _, _, _, _) => Some(1),
            Move::KnightMove(_, _, _, _, a) => Some(a.worth()),
            Move::BishopMove(_, _, _, _, a) => Some(a.worth()),
            Move::RookMove(_, _, _, _, a) => Some(a.worth()),
            Move::QueenMove(_, _, _, _, a) => Some(a.worth()),
            Move::KingMove(_, _, _, _, a) => Some(a.worth()),
            _ => None
        }
    }
}


// impl Ord for Move {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match (other.clone() as u8).cmp(&(self.clone() as u8)) {
//             Ordering::Equal => {
//                 if let Some(a) = self.get_captured_piece_worth() {
//                     if let Some(b) = other.get_captured_piece_worth() {
//                         a.cmp(&b)
//                     } else {
//                         Ordering::Greater
//                     }
//                 } else {
//                     Ordering::Less
//                 }
//             },
//             x => x
//         }
//     }
// }


// deja deja
impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::PawnPromotion(_, _, _, _, _, a), Move::PawnPromotion(_, _, _, _, _, b)) => a.compare(&b),
            (Move::PawnPromotion(_, _, _, _, _, _), _) => Ordering::Greater,
            (_, Move::PawnPromotion(_, _, _, _, _, _)) => Ordering::Less,
            (Move::PawnCapture(_, _, _ ,_, a), Move::PawnCapture(_, _, _ ,_, b)) => a.compare(&b),
            (Move::PawnCapture(_, _, _, _, _), _) => Ordering::Greater,
            (_, Move::PawnCapture(_, _, _, _, _)) => Ordering::Less,
            (Move::EnPassant(_, _, _, _, _), Move::EnPassant(_, _, _, _, _)) => Ordering::Equal,
            (Move::EnPassant(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::EnPassant(_, _, _, _, _)) => Ordering::Less,
            (Move::KnightMove(_, _, _, _, a), Move::KnightMove(_, _, _, _, b)) => a.compare(&b),
            (Move::KnightMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::KnightMove(_, _, _, _, _)) => Ordering::Less,
            (Move::BishopMove(_, _, _, _, a), Move::BishopMove(_, _, _, _, b)) => a.compare(&b),
            (Move::BishopMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::BishopMove(_, _, _, _, _)) => Ordering::Less,
            (Move::RookMove(_, _, _, _, a), Move::RookMove(_, _, _, _, b)) => a.compare(&b),
            (Move::RookMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::RookMove(_, _, _, _, _)) => Ordering::Less,
            (Move::QueenMove(_, _, _, _, a), Move::QueenMove(_, _, _, _, b)) => a.compare(&b),
            (Move::QueenMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::QueenMove(_, _, _, _, _)) => Ordering::Less,
            (Move::KingMove(_, _, _, _, a), Move::KingMove(_, _, _, _, b)) => a.compare(&b),
            (Move::KingMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::KingMove(_, _, _, _, _)) => Ordering::Less,
            (Move::PawnPush(_, _, _, _), Move::PawnPush(_, _, _, _)) => Ordering::Equal,
            (Move::PawnPush(_, _, _ ,_), _) => Ordering::Greater,
            (_, Move::PawnPush(_, _, _, _)) => Ordering::Less,
            (Move::TwoSquarePawnMove(_, _, _, _), Move::TwoSquarePawnMove(_, _, _, _)) => Ordering::Equal,
            (Move::TwoSquarePawnMove(_, _ ,_ ,_), _) => Ordering::Greater,
            (_, Move::TwoSquarePawnMove(_, _, _, _)) => Ordering::Less,
            _ => Ordering::Equal
        }
    }
}


impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Eq for Move {}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}


#[derive(Debug, Hash)]
pub struct Board {
    pub board: [[Square; 8]; 8],
    // white, black
    // on which move did the king first move
    pub king_moved: [usize; 2],
    // white kingside, white queenside, black kingside, black queenside
    // on which move did the rook first move
    pub rook_moved: [usize; 4],
    // stack memory is cheap
    // ifs are expensive
    // thus the en_passant_array was born
    pub en_passant_array: [[usize; 8]; 8],
    pub white_to_move: bool,
    // 50 moves without captures or pawn pushes is a draw
    pub halfmove_clock: usize,
    // which move (halfmove) it is
    pub clock: usize,
}

impl Board {
    pub fn from_fen(data: &str) -> Board {
        let mut splat = data.split_whitespace();
        let mut temp_board = [[Square::E; 8]; 8];
        let mut x = 0;
        let mut y = 0;
        for i in splat.next().unwrap().chars() {
            match i {
                '/' => {
                    x += 1;
                    y = 0
                }
                '8' => (),
                '7' => { y += 7 }
                '6' => { y += 6 }
                '5' => { y += 5 }
                '4' => { y += 4 }
                '3' => { y += 3 }
                '2' => { y += 2 }
                '1' => { y += 1 }
                c => {
                    temp_board[x][y] = Square::from_char(c);
                    y += 1
                }
            }
        }
        let white_to_move = splat.next().unwrap() == "w";
        let mut king_moved = [1, 1];
        let mut rook_moved = [1, 1, 1, 1];
        for i in splat.next().unwrap().chars() {
            match i {
                'K' => {
                    king_moved[0] = 0;
                    rook_moved[0] = 0
                }
                'Q' => {
                    king_moved[0] = 0;
                    rook_moved[1] = 0
                }
                'k' => {
                    king_moved[1] = 0;
                    rook_moved[2] = 0
                }
                'q' => {
                    king_moved[1] = 0;
                    rook_moved[3] = 0
                }
                _ => ()
            }
        }
        let raw_en_passant_string = splat.next().unwrap();
        let mut en_passant_array = [[0; 8]; 8];
        if raw_en_passant_string == "-" {
            Board {
                board: temp_board,
                king_moved,
                rook_moved,
                en_passant_array,
                white_to_move,
                halfmove_clock: splat.next().unwrap_or("0").parse().unwrap(),
                clock: 1,
            }
        } else {
            en_passant_array[56 - raw_en_passant_string.chars().nth(1).unwrap() as usize][raw_en_passant_string.chars().nth(0).unwrap() as usize - 97] = 1;
            Board {
                board: temp_board,
                king_moved,
                rook_moved,
                en_passant_array,
                white_to_move,
                halfmove_clock: splat.next().unwrap_or("0").parse().unwrap(),
                clock: 1,
            }
        }
    }
    pub fn from_starting_position() -> Board {
        Board {
            board: [
                [
                    Square::R { white: false },
                    Square::N { white: false },
                    Square::B { white: false },
                    Square::Q { white: false },
                    Square::K { white: false },
                    Square::B { white: false },
                    Square::N { white: false },
                    Square::R { white: false }
                ],
                [Square::P { white: false }; 8],
                [Square::E; 8],
                [Square::E; 8],
                [Square::E; 8],
                [Square::E; 8],
                [Square::P { white: true }; 8],
                [
                    Square::R { white: true },
                    Square::N { white: true },
                    Square::B { white: true },
                    Square::Q { white: true },
                    Square::K { white: true },
                    Square::B { white: true },
                    Square::N { white: true },
                    Square::R { white: true }
                ],
            ],
            king_moved: [0; 2],
            rook_moved: [0; 4],
            en_passant_array: [[0; 8]; 8],
            white_to_move: true,
            halfmove_clock: 0,
            clock: 1,
        }
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
    pub fn can_castle(&self, x: usize, b: usize, c: usize) -> bool {
        if !self.board[x][b].is_empty() || !self.board[x][c].is_empty() {
            return false;
        }
        if can_attack_square(&self, x, 4, self.white_to_move)
            || can_attack_square(&self, x, b, self.white_to_move)
            || can_attack_square(&self, x, c, self.white_to_move) {
            return false;
        }
        match (x, c) {
            (7, 6) => {
                match self.board[7][7] {
                    Square::R{white:true} => (),
                    _ => return false
                }
                match (self.board[6][3], self.board[6][4], self.board[6][5], self.board[6][6], self.board[6][7]) {
                    (Square::P{white:false}, _, _, _, _) |
                    (_, Square::P{white:false}, _, _, _) |
                    (_, _, Square::P{white:false}, _, _) |
                    (_, _, _, Square::P{white:false}, _) |
                    (_, _, _, Square::K{white:false}, _) |
                    (_, _, _, _, Square::P{white:false}) => return false,
                    _ => ()
                }
            },
            (7, 2) => {
                match self.board[7][0] {
                    Square::R{white:true} => (),
                    _ => return false
                }
                match (self.board[6][5], self.board[6][4], self.board[6][3], self.board[6][2], self.board[6][1]) {
                    (Square::P{white:false}, _, _, _, _) |
                    (_, Square::P{white:false}, _, _, _) |
                    (_, _, Square::P{white:false}, _, _) |
                    (_, _, _, Square::P{white:false}, _) |
                    (_, _, _, Square::K{white:false}, _) |
                    (_, _, _, _, Square::P{white:false}) |
                    (_, _, _, _, Square::K{white:false}) => return false,
                    _ => ()
                }
            },
            (0, 6) => {
                match self.board[0][7] {
                    Square::R{white:false} => (),
                    _ => return false
                }
                match (self.board[1][3], self.board[1][4], self.board[1][5], self.board[1][6], self.board[1][7]) {
                    (Square::P{white:true}, _, _, _, _) |
                    (_, Square::P{white:true}, _, _, _) |
                    (_, _, Square::P{white:true}, _, _) |
                    (_, _, _, Square::P{white:true}, _) |
                    (_, _, _, Square::K{white:true}, _) |
                    (_, _, _, _, Square::P{white:true}) => return false,
                    _ => ()
                }
            },
            (0, 2) => {
                match self.board[0][0] {
                    Square::R{white:false} => (),
                    _ => return false
                }
                match (self.board[1][5], self.board[1][4], self.board[1][3], self.board[1][2], self.board[1][1]) {
                    (Square::P{white:true}, _, _, _, _) |
                    (_, Square::P{white:true}, _, _, _) |
                    (_, _, Square::P{white:true}, _, _) |
                    (_, _, _, Square::P{white:true}, _) |
                    (_, _, _, Square::K{white:true}, _) |
                    (_, _, _, _, Square::P{white:true}) |
                    (_, _, _, _, Square::K{white:true}) => return false,
                    _ => ()
                }
            },
            _ => ()
        }
        true
    }
    // TODO halfmove clock is not used
    pub fn make_move(&mut self, possible_move: &Move) {
        self.clock += 1;
        match possible_move {
            Move::EnPassant(a, b, c, d, _) => {
                self.board[*a][*d] = Square::E;
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            // kingside castling
            Move::Castling(0, _, _, 6) => {
                self.board[0][6] = self.board[0][4];
                self.board[0][4] = Square::E;
                self.board[0][5] = self.board[0][7];
                self.board[0][7] = Square::E;
                self.king_moved[1] = self.clock;
            }
            Move::Castling(_, _, _, 6) => {
                self.board[7][6] = self.board[7][4];
                self.board[7][4] = Square::E;
                self.board[7][5] = self.board[7][7];
                self.board[7][7] = Square::E;
                self.king_moved[0] = self.clock;
            }
            // queenside castling
            Move::Castling(0, _, _, _) => {
                self.board[0][2] = self.board[0][4];
                self.board[0][4] = Square::E;
                self.board[0][3] = self.board[0][0];
                self.board[0][0] = Square::E;
                self.king_moved[1] = self.clock;
            }
            Move::Castling(_, _, _, _) => {
                self.board[7][2] = self.board[7][4];
                self.board[7][4] = Square::E;
                self.board[7][3] = self.board[7][0];
                self.board[7][0] = Square::E;
                self.king_moved[0] = self.clock;
            }
            Move::KingMove(a, b, c, d, _) => {
                match (a, b) {
                    (0, 4) => {
                        if self.king_moved[1] == 0 {
                            self.king_moved[1] = self.clock;
                        }
                    }
                    (7, 4) => {
                        if self.king_moved[0] == 0 {
                            self.king_moved[0] = self.clock;
                        }
                    }
                    _ => ()
                };
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::RookMove(a, b, c, d, _) => {
                match (a, b) {
                    (7, 7) => if self.rook_moved[0] == 0 {
                        self.rook_moved[0] = self.clock;
                    },
                    (7, 0) => if self.rook_moved[1] == 0 {
                        self.rook_moved[1] = self.clock;
                    },
                    (0, 7) => if self.rook_moved[2] == 0 {
                        self.rook_moved[2] = self.clock;
                    },
                    (0, 0) => if self.rook_moved[3] == 0 {
                        self.rook_moved[3] = self.clock;
                    },
                    _ => ()
                }
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::TwoSquarePawnMove(1, b, _, _) => {
                self.board[3][*b] = self.board[1][*b];
                self.board[1][*b] = Square::E;
                self.en_passant_array[2][*b] = self.clock + 1;
            }
            Move::TwoSquarePawnMove(_, b, _, _) => {
                self.board[4][*b] = self.board[6][*b];
                self.board[6][*b] = Square::E;
                self.en_passant_array[5][*b] = self.clock + 1;
            }
            Move::PawnPromotion(a, b, c, d, _, p) => {
                self.board[*c][*d] = *p;
                self.board[*a][*b] = Square::E;
            }
            Move::PawnCapture(a, b, c, d, _) => {
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::PawnPush(a, b , c, d) => {
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::KnightMove(a, b, c, d, _) => {
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::BishopMove(a, b, c, d, _) => {
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
            Move::QueenMove(a, b, c, d, _) => {
                self.board[*c][*d] = self.board[*a][*b];
                self.board[*a][*b] = Square::E;
            }
        }
        self.white_to_move = !self.white_to_move;
    }

    // TODO halfmove clock is not used
    pub fn undo_move(&mut self, previous_move: &Move) {
        match previous_move {
            Move::EnPassant(a, b, c, d, p) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = Square::E;
                self.board[*a][*d] = *p;
                self.en_passant_array[*c][*d] = self.clock - 1;
            }
            // kingside castling
            Move::Castling(0, _, _, 6) => {
                self.board[0][4] = self.board[0][6];
                self.board[0][6] = Square::E;
                self.board[0][7] = self.board[0][5];
                self.board[0][5] = Square::E;
                self.king_moved[1] = 0;
            }
            Move::Castling(_, _, _, 6) => {
                self.board[7][4] = self.board[7][6];
                self.board[7][6] = Square::E;
                self.board[7][7] = self.board[7][5];
                self.board[7][5] = Square::E;
                self.king_moved[0] = 0;
            }
            // queenside castling
            Move::Castling(0, _, _, _) => {
                self.board[0][4] = self.board[0][2];
                self.board[0][2] = Square::E;
                self.board[0][0] = self.board[0][3];
                self.board[0][3] = Square::E;
                self.king_moved[1] = 0;
            }
            Move::Castling(_, _, _, _) => {
                self.board[7][4] = self.board[7][2];
                self.board[7][2] = Square::E;
                self.board[7][0] = self.board[7][3];
                self.board[7][3] = Square::E;
                self.king_moved[0] = 0;
            }
            Move::KingMove(a, b, c, d, p) => {
                match (a, b) {
                    (0, 4) => {
                        if self.king_moved[1] == self.clock {
                            self.king_moved[1] = 0;
                        }
                    }
                    (7, 4) => {
                        if self.king_moved[0] == self.clock {
                            self.king_moved[0] = 0;
                        }
                    }
                    _ => ()
                }
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
            Move::RookMove(a, b, c, d, p) => {
                match (a, b) {
                    (7, 7) => if self.rook_moved[0] == self.clock {
                        self.rook_moved[0] = 0;
                    },
                    (7, 0) => if self.rook_moved[1] == self.clock {
                        self.rook_moved[1] = 0;
                    },
                    (0, 7) => if self.rook_moved[2] == self.clock {
                        self.rook_moved[2] = 0;
                    },
                    (0, 0) => if self.rook_moved[3] == self.clock {
                        self.rook_moved[3] = 0;
                    },
                    _ => ()
                }
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
            Move::TwoSquarePawnMove(1, b, _, _) => {
                self.board[1][*b] = self.board[3][*b];
                self.board[3][*b] = Square::E;
                self.en_passant_array[2][*b] = 0;
            }
            Move::TwoSquarePawnMove(_, b, _, _) => {
                self.board[6][*b] = self.board[4][*b];
                self.board[4][*b] = Square::E;
                self.en_passant_array[5][*b] = 0;
            }
            Move::PawnPromotion(1, b, 0, d, p, _) => {
                self.board[0][*d] = *p;
                self.board[1][*b] = Square::P{white: true};
            }
            Move::PawnPromotion(_, b, _, d, p, _) => {
                self.board[7][*d] = *p;
                self.board[6][*b] = Square::P{white: false};
            }
            Move::PawnCapture(a, b, c, d, p) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
            Move::PawnPush(a, b, c, d) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = Square::E;
            }
            Move::KnightMove(a, b, c, d, p) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
            Move::BishopMove(a, b, c, d, p) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
            Move::QueenMove(a, b, c, d, p) => {
                self.board[*a][*b] = self.board[*c][*d];
                self.board[*c][*d] = *p;
            }
        }
        self.clock -= 1;
        self.white_to_move = !self.white_to_move;
    }

    pub fn display_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{}", self.board[i][j].display_square())
            }
            println!();
        }
    }

    pub fn display_debug_info(&self) {
        println!("rook_moved: {:?}", self.rook_moved);
        println!("king_moved: {:?}", self.king_moved);
        println!("en_passant_array: {:?}", self.en_passant_array);
        println!("white_to_move: {:?}", self.white_to_move);
        println!("clock: {:?}", self.clock);
        println!("halfmove_clock: {:?}", self.halfmove_clock);
    }

    pub fn to_fen(&self) -> String {
        let mut counter = 0;
        let mut string = String::new();
        for i in 0..8 {
            for j in 0..8 {
                match self.board[i][j] {
                    Square::E => {counter += 1},
                    Square::P{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('P')},
                    Square::P{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('p')},
                    Square::N{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('N')},
                    Square::N{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('n')},
                    Square::B{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('B')},
                    Square::B{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('b')},
                    Square::R{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('R')},
                    Square::R{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('r')},
                    Square::Q{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('Q')},
                    Square::Q{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('q')},
                    Square::K{white:true} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('K')},
                    Square::K{white:false} => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('k')}
                }
            }
            if counter != 0 {
                string.push(char::from_digit(counter, 10).unwrap());
                counter = 0;
            }
            string.push('/');
        }
        string.pop();
        if self.white_to_move {
            string.push_str(" w ");
        } else {
            string.push_str(" b ");
        }
        if self.king_moved[0] == 0 {
            if self.rook_moved[0] == 0 {
                string.push('K');
            }
            if self.rook_moved[1] == 0 {
                string.push('Q');
            }
        }
        if self.king_moved[1] == 0 {
            if self.rook_moved[2] == 0 {
                string.push('k');
            }
            if self.rook_moved[3] == 0 {
                string.push('q');
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                if self.en_passant_array[i][j] == self.clock {
                    string.push(' ');
                    string.push(char::from_u32(97 + j as u32).unwrap());
                    string.push(char::from_u32(56 - i as u32).unwrap());
                    // the rest is not important lol
                    return string
                }
            }
        }
        string.push_str(" -");
        string
    }
}



pub fn generate_into(board: &Board, vector: &mut Vec<Move>) {
    vector.clear();
    for x in 0..8 {
        for y in 0..8 {
            match board.board[x][y] {
                Square::P { white } if white == board.white_to_move => {
                    generate_pawn_moves(board, x, y, vector);
                },
                Square::N { white } if white == board.white_to_move => {
                    generate_knight_moves(board, x, y, vector);
                },
                Square::B { white } if white == board.white_to_move => {
                    generate_bishop_moves(board, x, y, vector);
                },
                Square::R { white } if white == board.white_to_move => {
                    generate_rook_moves(board, x, y, vector);
                },
                Square::Q { white } if white == board.white_to_move => {
                    generate_queen_moves(board, x, y, vector);
                },
                Square::K { white } if white == board.white_to_move => {
                    generate_king_moves(board, x, y, vector);
                },
                _ => ()
            }
        }
    }
}

pub fn generate_captures_into(board: &Board, vector: &mut Vec<Move>) {
    vector.clear();
    for x in 0..8 {
        for y in 0..8 {
            match board.board[x][y] {
                Square::P { white } if white == board.white_to_move => {
                    generate_pawn_captures(board, x, y, vector);
                },
                Square::N { white } if white == board.white_to_move => {
                    generate_knight_captures(board, x, y, vector);
                },
                Square::B { white } if white == board.white_to_move => {
                    generate_bishop_captures(board, x, y, vector);
                },
                Square::R { white } if white == board.white_to_move => {
                    generate_rook_captures(board, x, y, vector);
                },
                Square::Q { white } if white == board.white_to_move => {
                    generate_queen_captures(board, x, y, vector);
                },
                Square::K { white } if white == board.white_to_move => {
                    generate_king_captures(board, x, y, vector);
                },
                _ => ()
            }
        }
    }
}
