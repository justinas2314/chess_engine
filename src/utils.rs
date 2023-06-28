use crate::move_gen::*;
use crate::eval::{MATERIAL_WORTH, PIECE_DELTA, PIECE_SQUARE_TABLE};
use std::hash::Hash;
use std::collections::HashMap;
// use crate::debug::{find_buggy_visibility, find_buggy_visibility_other};

pub struct PositionStorer {
    first_hashmap: HashMap<u64, u8>,
    second_hashmap: HashMap<u64, u8>,
    bolevic_hashmap: HashMap<u64, u8>,
    pub accent_repetitions: bool
}

impl PositionStorer {
    pub fn new() -> PositionStorer {
        PositionStorer {
            first_hashmap: HashMap::with_capacity(200),
            second_hashmap: HashMap::with_capacity(200),
            bolevic_hashmap: HashMap::with_capacity(200),
            accent_repetitions: false
        }
    }
    pub fn add_first_position(&mut self, hash: u64) {
        let mut res = self.first_hashmap.get(&hash);
        if let Some(count) = res {
            self.first_hashmap.insert(hash, count + 1);
        } else {
            self.first_hashmap.insert(hash, 1);
        }
        res = self.bolevic_hashmap.get(&hash);
        if let Some(count) = res {
            self.bolevic_hashmap.insert(hash, count + 1);
        } else {
            self.bolevic_hashmap.insert(hash, 1);
        }
    }
    pub fn add_second_position(&mut self, hash: u64) {
        let mut res = self.second_hashmap.get(&hash);
        if let Some(count) = res {
            self.second_hashmap.insert(hash, count + 1);
        } else {
            self.second_hashmap.insert(hash, 1);
        }
        res = self.bolevic_hashmap.get(&hash);
        if let Some(count) = res {
            self.bolevic_hashmap.insert(hash, count + 1);
        } else {
            self.bolevic_hashmap.insert(hash, 1);
        }
    }
    pub fn update_with_draws(&self, cache: &mut HashMapReuser<u64, i64>) {
        // pasirodo tam tikras nedas bolevicius laiko dvi pozicijas
        // identiskom net kai ir skirtingos puses ejimas
        let mut index;
        for (k, _) in self.bolevic_hashmap.iter().filter(|(_, v)| **v > 1) {
            index = 0;
            while index < cache.hashmaps.len() {
                // issa draw
                cache.insert(index, *k, 0);
                index += 1;
            }
        }
        if self.accent_repetitions {
            for (k, _) in self.first_hashmap.iter().filter(|(_, v)| **v == 1) {
                index = 1;
                while index < cache.hashmaps.len() {
                    cache.insert(index, *k, 0);
                    index += 2;
                }
            }
            for (k, _) in self.second_hashmap.iter().filter(|(_, v)| **v == 1) {
                index = 0;
                while index < cache.hashmaps.len() {
                    cache.insert(index, *k, 0);
                    index += 2;
                }
            }
        }
    }
}

pub struct BestMoves {
    // nesakykit simonaviciui kad naudoju publik
    pub moves: [usize; 3],
    insert_index: usize,
    get_index: usize
}

impl BestMoves {
    pub fn new() -> BestMoves {
        BestMoves {
            moves: [4096, 4096, 4096],
            insert_index: 0,
            get_index: 0
        }
    }
    pub fn insert(&mut self, new_move: usize) {
        self.moves[self.insert_index] = new_move;
        self.insert_index += 1;
        self.insert_index %= 3;
    }
    pub fn get(&mut self) -> usize {
        self.get_index += 1;
        self.get_index %= 3;
        self.moves[self.get_index]
    }
}


pub struct VectorReuser<T> {
    vectors: Vec<Vec<T>>,
    capacity: usize
}

impl<T> VectorReuser<T> {
    pub fn with_capacity2d(length: usize, capacity: usize) -> VectorReuser<T> {
        let mut inner_vector = Vec::with_capacity(length);
        for _ in 0..length {
            inner_vector.push(Vec::with_capacity(capacity));
        }
        VectorReuser {
            vectors: inner_vector,
            capacity
        }
    }
    pub fn with_capacity(capacity: usize) -> VectorReuser<T> {
        VectorReuser {
            vectors: Vec::with_capacity(50),
            capacity
        }
    }
    // sitas API yra ziauriai unsafe ir niekad runninant koda 100% nezinau ar jis
    // neproducins undefined behaviour ... BET ... tingiu perrasinet koda
    pub fn get_vector_pointer(&mut self, index: usize) -> *mut Vec<T> {
        debug_assert!(index < self.vectors.capacity());
        // if self.vectors.get(index).is_none() {
        //     self.vectors.push(Vec::with_capacity(self.capacity));
        // }
        &mut self.vectors[index]
    }
}

pub struct HashMapReuser<K: Eq + Hash, V> {
    pub hashmaps: Vec<HashMap<K, V>>,
    capacity: usize
}

impl<K: Eq + Hash, V> HashMapReuser<K, V> {
    pub fn with_capacity2d(length: usize, capacity: usize) -> HashMapReuser<K, V> {
        let mut inner_vector = Vec::with_capacity(length);
        for _ in 0..length {
            inner_vector.push(HashMap::with_capacity(capacity));
        }
        HashMapReuser {
            hashmaps: inner_vector,
            capacity
        }
    }
    pub fn with_capacity(capacity: usize) -> HashMapReuser<K, V> {
        HashMapReuser {
            hashmaps: Vec::new(),
            capacity
        }
    }
    pub fn insert(&mut self, index: usize, key: K, value: V) {
        debug_assert!(self.hashmaps[index].len() < self.capacity);
        // if self.hashmaps.get(index).is_none() {
        //     self.hashmaps.push(HashMap::with_capacity(self.capacity));
        // }
        self.hashmaps[index].insert(key, value);
    }
    pub fn get(&mut self, index: usize, key: &K) -> Option<&V> {
        // if index == self.hashmaps.len() {
        //     self.hashmaps.push(HashMap::with_capacity(self.capacity));
        // }
        // println!("size: {}", self.hashmaps[index].len());
        self.hashmaps[index].get(key)
    }
    pub fn clear(&mut self) {
        for i in self.hashmaps.iter_mut() {
            i.clear();
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Square {
    E = 0,
    PW = 1,
    NW = 2,
    BW = 3,
    RW = 4,
    QW = 5,
    KW = 6,
    // + 6 to give the n word pass
    PB = 7,
    NB = 8,
    BB = 9,
    RB = 10,
    QB = 11,
    KB = 12
}

// #[derive(Debug, Copy, Clone)]
// pub enum Square {
//     E,
//     P { white: bool },
//     N { white: bool },
//     B { white: bool },
//     R { white: bool },
//     Q { white: bool },
//     K { white: bool },
// }

impl Square {
    fn from_char(c: char) -> Square {
        match c {
            'P' => Square::PW,
            'p' => Square::PB,
            'N' => Square::NW,
            'n' => Square::NB,
            'B' => Square::BW,
            'b' => Square::BB,
            'R' => Square::RW,
            'r' => Square::RB,
            'Q' => Square::QW,
            'q' => Square::QB,
            'K' => Square::KW,
            'k' => Square::KB,
            _ => Square::E
        }
    }
    fn display_square(&self) -> char {
        match self {
            Square::E => '.',
            Square::PW => 'P',
            Square::PB => 'p',
            Square::NW => 'N',
            Square::NB => 'n',
            Square::BW => 'B',
            Square::BB => 'b',
            Square::RW => 'R',
            Square::RB => 'r',
            Square::QW => 'Q',
            Square::QB => 'q',
            Square::KW => 'K',
            Square::KB => 'k'
        }
    }
    pub fn worth(&self) -> i64 {
        match self {
            Square::E => 0,
            Square::PW | Square::PB => 1,
            Square::NW | Square::NB => 3,
            Square::BW | Square::BB => 3,
            Square::RW | Square::RB => 5,
            Square::QW | Square::QB => 9,
            Square::KW | Square::KB => 10_000
        }
    }
    pub fn pawn(&self) -> bool {
        match self {
            Square::PW | Square::PB => true,
            _ => false
        }
    }
    pub fn piece(&self) -> bool {
        match self {
            Square::E | Square::PW | Square::PB => false,
            _ => true
        }
    }
    pub fn white_piece(&self) -> bool {
        match self {
            Square::NW => true,
            Square::BW => true,
            Square::RW => true,
            Square::QW => true,
            Square::KW => true,
            _ => false
        }
    }
    pub fn black_piece(&self) -> bool {
        match self {
            Square::NB => true,
            Square::BB => true,
            Square::RB => true,
            Square::QB => true,
            Square::KB => true,
            _ => false
        }
    }
    pub fn bishopy_enemy(&self, white_move: bool) -> usize {
        if white_move {
            match self {
                Square::E => 1,
                Square::BB => 2,
                Square::QB => 2,
                _ => 0
            }
        } else {
            match self {
                Square::E => 1,
                Square::BW => 2,
                Square::QW => 2,
                _ => 0
            }
        }
    }
    pub fn bishopy(&self) -> bool {
        match self {
            Square::BW | Square::BB => true,
            Square::QW | Square::QB => true,
            _ => false
        }
    }
    pub fn rooky_enemy(&self, white_move: bool) -> usize {
        if white_move {
            match self {
                Square::E => 1,
                Square::RB => 2,
                Square::QB => 2,
                _ => 0
            }
        } else {
            match self {
                Square::E => 1,
                Square::RW => 2,
                Square::QW => 2,
                _ => 0
            }
        }
    }
    pub fn rooky(&self) -> bool {
        match self {
            Square::RW | Square::RB => true,
            Square::QW | Square::QB => true,
            _ => false
        }
    }
    // more like can_be_captured_by
    pub fn can_capture(&self, white_move: bool) -> bool {
        if white_move {
            match self {
                Square::PB => true,
                Square::NB => true,
                Square::BB => true,
                Square::RB => true,
                Square::QB => true,
                Square::KB => true,
                _ => false
            }
        } else {
            match self {
                Square::PW => true,
                Square::NW => true,
                Square::BW => true,
                Square::RW => true,
                Square::QW => true,
                Square::KW => true,
                _ => false
            }
        }
    }
    pub fn can_move_to(&self, white_move: bool) -> bool {
        if white_move {
            match self {
                Square::E => true,
                Square::PB => true,
                Square::NB => true,
                Square::BB => true,
                Square::RB => true,
                Square::QB => true,
                Square::KB => true,
                _ => false
            }
        } else {
            match self {
                Square::E => true,
                Square::PW => true,
                Square::NW => true,
                Square::BW => true,
                Square::RW => true,
                Square::QW => true,
                Square::KW => true,
                _ => false
            }
        }
    }
    pub fn can_move_to_and_through(&self, white_move: bool) -> (bool, bool) {
        if white_move {
            match self {
                Square::E => (true, true),
                Square::PB => (true, false),
                Square::NB => (true, false),
                Square::BB => (true, false),
                Square::RB => (true, false),
                Square::QB => (true, false),
                Square::KB => (true, false),
                _ => (false, false)
            }
        } else {
            match self {
                Square::E => (true, true),
                Square::PW => (true, false),
                Square::NW => (true, false),
                Square::BW => (true, false),
                Square::RW => (true, false),
                Square::QW => (true, false),
                Square::KW => (true, false),
                _ => (false, false)
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Square::E => true,
            _ => false
        }
    }
    pub fn is_white(&self) -> bool {
        // empty is not white
        match self {
            Square::PW => true,
            Square::NW => true,
            Square::BW => true,
            Square::RW => true,
            Square::QW => true,
            Square::KW => true,
            _ => false
        }
    }
}


#[derive(Debug, Clone, Copy)]
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
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        match self {
            Move::PawnPromotion(a, b, c, d, _, p) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
                match p {
                    Square::QW | Square::QB => output.push('q'),
                    Square::RW | Square::RB => output.push('r'),
                    Square::BW | Square::BB => output.push('b'),
                    Square::NW | Square::NB => output.push('n'),
                    _ => panic!("inside to_string\nare we promoting to a king?")
                }
            },
            Move::PawnCapture(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::EnPassant(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::KnightMove(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::BishopMove(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::RookMove(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::QueenMove(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::KingMove(a, b, c, d, _) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::PawnPush(a, b, c, d) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::TwoSquarePawnMove(a, b, c, d) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            },
            Move::Castling(a, b, c, d) => {
                output.push("abcdefgh".chars().nth(*b).unwrap());
                output.push_str(&(8 - *a).to_string());
                output.push("abcdefgh".chars().nth(*d).unwrap());
                output.push_str(&(8 - *c).to_string());
            }
        }
        output
    }
    pub fn to_u8(&self) -> u8 {
        // changes which moves will be checked first
        match self {
            Move::PawnPromotion(_, _, _, _, _, _) => 200,
            Move::PawnCapture(_, _, _, _, _) => 150,
            Move::EnPassant(_, _, _, _, _) => 250,
            Move::KnightMove(_, _, _, _, _) => 100,
            Move::BishopMove(_, _, _, _, _) => 80,
            Move::RookMove(_, _, _, _, _) => 60,
            Move::QueenMove(_, _, _, _, _) => 40,
            Move::KingMove(_, _, _, _, _) => 10,
            Move::PawnPush(_, _, _, _) => 5,
            Move::TwoSquarePawnMove(_, _, _, _) => 20,
            Move::Castling(_, _, _, _) => 0
        }
    }
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
    pub fn get_from_xy(&self) -> (usize, usize) {
        match self {
            Move::PawnPromotion(x, y, _, _, _, _) => (*x, *y),
            Move::PawnCapture(x, y, _, _, _) => (*x, *y),
            Move::EnPassant(x, y, _, _, _) => (*x, *y),
            Move::KnightMove(x, y, _, _, _) => (*x, *y),
            Move::BishopMove(x, y, _, _, _) => (*x, *y),
            Move::RookMove(x, y, _, _, _) => (*x, *y),
            Move::QueenMove(x, y, _, _, _) => (*x, *y),
            Move::KingMove(x, y, _, _, _) => (*x, *y),
            Move::PawnPush(x, y, _, _) => (*x, *y),
            Move::TwoSquarePawnMove(x, y, _, _) => (*x, *y),
            Move::Castling(x, y, _, _) => (*x, *y)
        }
    }
    pub fn get_captured_piece_worth(&self) -> Option<i64> {
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
    pub fn get_piece_hash(&self, board: &Board) -> usize {
        let (a, b, c, d) = match *self {
            Move::PawnPromotion(a, b, c, d, _, _) => (a, b, c, d),
            Move::PawnCapture(a, b, c, d, _) => (a, b, c, d),
            Move::EnPassant(a, b, c, d, _) => (a, b, c, d),
            Move::KnightMove(a, b, c, d, _) => (a, b, c, d),
            Move::BishopMove(a, b, c, d, _) => (a, b, c, d),
            Move::RookMove(a, b, c, d, _) => (a, b, c, d),
            Move::QueenMove(a, b, c, d, _) => (a, b, c, d),
            Move::KingMove(a, b, c, d, _) => (a, b, c, d),
            Move::PawnPush(a, b, c, d) => (a, b, c, d),
            Move::TwoSquarePawnMove(a, b, c, d) => (a, b, c, d),
            Move::Castling(a, b, c, d) => (a, b, c, d)
        };
        (((board.board[a][b] as usize) << 3) | c) << 3 | d
        // (((((a << 3) | b) << 3) | c) << 3) | d
    }
    pub fn get_coord_hash(&self) -> usize {
        let (a, b, c, d) = match *self {
            Move::PawnPromotion(a, b, c, d, _, _) => (a, b, c, d),
            Move::PawnCapture(a, b, c, d, _) => (a, b, c, d),
            Move::EnPassant(a, b, c, d, _) => (a, b, c, d),
            Move::KnightMove(a, b, c, d, _) => (a, b, c, d),
            Move::BishopMove(a, b, c, d, _) => (a, b, c, d),
            Move::RookMove(a, b, c, d, _) => (a, b, c, d),
            Move::QueenMove(a, b, c, d, _) => (a, b, c, d),
            Move::KingMove(a, b, c, d, _) => (a, b, c, d),
            Move::PawnPush(a, b, c, d) => (a, b, c, d),
            Move::TwoSquarePawnMove(a, b, c, d) => (a, b, c, d),
            Move::Castling(a, b, c, d) => (a, b, c, d)
        };
        (((((a << 3) | b) << 3) | c) << 3) | d
    }
    pub fn get_capture_delta(&self) -> i64 {
        match self {
            Move::PawnPromotion(_, _, _, _, a, b) => PIECE_DELTA[*a as usize] + PIECE_DELTA[*b as usize],
            Move::PawnCapture(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::EnPassant(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::KnightMove(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::BishopMove(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::RookMove(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::QueenMove(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            Move::KingMove(_, _, _, _, p) => PIECE_DELTA[*p as usize],
            _ => PIECE_DELTA[0]
        }
    }
    pub fn is_capture(&self) -> bool {
        match self {
            // Move::PawnPromotion(_, _, _, _, p, _) => !p.is_empty(),
            Move::PawnPromotion(_, _, _, _, _, _) => true,
            Move::PawnCapture(_, _, _, _, _) => true,
            Move::EnPassant(_, _, _, _, _) => true,
            Move::KnightMove(_, _, _, _, p) => !p.is_empty(),
            Move::BishopMove(_, _, _, _, p) => !p.is_empty(),
            Move::RookMove(_, _, _, _, p) => !p.is_empty(),
            Move::QueenMove(_, _, _, _, p) => !p.is_empty(),
            Move::KingMove(_, _, _, _, p) => !p.is_empty(),
            _ => false
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
// impl Ord for Move {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match (self, other) {
//             (Move::PawnPromotion(_, _, _, _, _, _), Move::PawnPromotion(_, _, _, _, _, _)) => Ordering::Equal,
//             (Move::PawnPromotion(_, _, _, _, _, _), _) => Ordering::Greater,
//             (_, Move::PawnPromotion(_, _, _, _, _, _)) => Ordering::Less,
//             (Move::PawnCapture(_, _, _ ,_, _), Move::PawnCapture(_, _, _ ,_, _)) => Ordering::Equal,
//             (Move::PawnCapture(_, _, _, _, _), _) => Ordering::Greater,
//             (_, Move::PawnCapture(_, _, _, _, _)) => Ordering::Less,
//             (Move::EnPassant(_, _, _, _, _), Move::EnPassant(_, _, _, _, _)) => Ordering::Equal,
//             (Move::EnPassant(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::EnPassant(_, _, _, _, _)) => Ordering::Less,
//             (Move::KnightMove(_, _, _, _, _), Move::KnightMove(_, _, _, _, _)) => Ordering::Equal,
//             (Move::KnightMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::KnightMove(_, _, _, _, _)) => Ordering::Less,
//             (Move::BishopMove(_, _, _, _, _), Move::BishopMove(_, _, _, _, _)) => Ordering::Equal,
//             (Move::BishopMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::BishopMove(_, _, _, _, _)) => Ordering::Less,
//             (Move::RookMove(_, _, _, _, _), Move::RookMove(_, _, _, _, _)) => Ordering::Equal,
//             (Move::RookMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::RookMove(_, _, _, _, _)) => Ordering::Less,
//             (Move::QueenMove(_, _, _, _, _), Move::QueenMove(_, _, _, _, _)) => Ordering::Equal,
//             (Move::QueenMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::QueenMove(_, _, _, _, _)) => Ordering::Less,
//             (Move::KingMove(_, _, _, _, _), Move::KingMove(_, _, _, _, _)) => Ordering::Equal,
//             (Move::KingMove(_, _, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::KingMove(_, _, _, _, _)) => Ordering::Less,
//             (Move::PawnPush(_, _, _, _), Move::PawnPush(_, _, _, _)) => Ordering::Equal,
//             (Move::PawnPush(_, _, _ ,_), _) => Ordering::Greater,
//             (_, Move::PawnPush(_, _, _, _)) => Ordering::Less,
//             (Move::TwoSquarePawnMove(_, _, _, _), Move::TwoSquarePawnMove(_, _, _, _)) => Ordering::Equal,
//             (Move::TwoSquarePawnMove(_, _ ,_ ,_), _) => Ordering::Greater,
//             (_, Move::TwoSquarePawnMove(_, _, _, _)) => Ordering::Less,
//             _ => Ordering::Equal
//         }
//     }
// }

//
// impl PartialOrd for Move {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }


// impl Eq for Move {}
//
// impl PartialEq for Move {
//     fn eq(&self, other: &Self) -> bool {
//         // only check if the square coordinates match
//         // move type and piece captured/promoted are not considered
//         self.get_hash() == other.get_hash()
//     }
// }
//
// impl Hash for Move {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.get_hash().hash(state);
//     }
// }


#[derive(Debug)]
pub struct Board {
    pub board: [[Square; 8]; 8],
    pub kings: [[usize; 2]; 2],
    // white, black
    // on which move did the king first move
    pub king_moved: [usize; 2],
    // white kingside, white queenside, black kingside, black queenside
    // on which move did the rook first move
    pub rook_moved: [usize; 4],
    // [total, pawn, knight, bishop, rook, queen, king]
    pub visibility: Visibility,
    // stack memory is cheap
    // ifs are expensive
    // thus the en_passant_array was born
    pub en_passant_array: [[usize; 8]; 8],
    pub en_passant_hash_index: [usize; 1000],
    pub white_to_move: bool,
    // 50 moves without captures or pawn pushes is a draw
    pub halfmove_clock: usize,
    // which move (halfmove) it is
    pub clock: usize,
    // updated incrementally
    pub evaluation: Evaluation
}

impl Board {
    pub fn from_fen(data: &str) -> Board {
        let mut splat = data.split_whitespace();
        let mut temp_board = [[Square::E; 8]; 8];
        let mut x = 0;
        let mut y = 0;
        let mut kings = [[0, 0], [0, 0]];
        for i in splat.next().unwrap().chars() {
            match i {
                '/' => {
                    x += 1;
                    y = 0
                }
                '8' => (),
                '7' => { y += 7 },
                '6' => { y += 6 },
                '5' => { y += 5 },
                '4' => { y += 4 },
                '3' => { y += 3 },
                '2' => { y += 2 },
                '1' => { y += 1 },
                'K' => {
                    temp_board[x][y] = Square::KW;
                    kings[0] = [x, y];
                    y += 1
                },
                'k' => {
                    temp_board[x][y] = Square::KB;
                    kings[1] = [x, y];
                    y += 1
                },
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
        let mut en_passant_hash_index = [8; 1000];
        let visibility = Visibility::new(&temp_board);
        let evaluation = Evaluation::new(&temp_board);
        if raw_en_passant_string == "-" {
            Board {
                board: temp_board,
                king_moved,
                kings,
                rook_moved,
                visibility,
                evaluation,
                en_passant_array,
                en_passant_hash_index,
                white_to_move,
                halfmove_clock: splat.next().unwrap_or("0").parse().unwrap(),
                clock: 1,
            }
        } else {
            en_passant_array[56 - raw_en_passant_string.chars().nth(1).unwrap() as usize][raw_en_passant_string.chars().nth(0).unwrap() as usize - 97] = 1;
            en_passant_hash_index[1] = raw_en_passant_string.chars().nth(0).unwrap() as usize - 97;
            Board {
                board: temp_board,
                king_moved,
                kings,
                rook_moved,
                visibility,
                evaluation,
                en_passant_array,
                en_passant_hash_index,
                white_to_move,
                halfmove_clock: splat.next().unwrap_or("0").parse().unwrap(),
                clock: 1,
            }
        }
    }
    pub fn from_starting_position() -> Board {
        let inner_board = [
            [
                Square::RB,
                Square::NB,
                Square::BB,
                Square::QB,
                Square::KB,
                Square::BB,
                Square::NB,
                Square::RB
            ],
            [Square::PB; 8],
            [Square::E; 8],
            [Square::E; 8],
            [Square::E; 8],
            [Square::E; 8],
            [Square::PW; 8],
            [
                Square::RW,
                Square::NW,
                Square::BW,
                Square::QW,
                Square::KW,
                Square::BW,
                Square::NW,
                Square::RW
            ],
        ];
        Board {
            board: inner_board,
            visibility: Visibility::new(&inner_board),
            evaluation: Evaluation::default(),
            kings: [[7, 4], [0, 4]],
            king_moved: [0; 2],
            rook_moved: [0; 4],
            en_passant_array: [[0; 8]; 8],
            en_passant_hash_index: [8; 1000],
            white_to_move: true,
            halfmove_clock: 0,
            clock: 1,
        }
    }

    pub fn update_visibility_before_move(&mut self, sx: usize, sy: usize, tx: usize, ty: usize) {
        if self.board[sx][sy].is_white() {
            self.visibility.clear_piece_from_existence(0, sx, sy);
            if !self.board[tx][ty].is_empty() {
                self.visibility.clear_piece_from_existence(1, tx, ty);
            }
        } else {
            self.visibility.clear_piece_from_existence(1, sx, sy);
            if !self.board[tx][ty].is_empty() {
                self.visibility.clear_piece_from_existence(0, tx, ty);
            }
        }
        // find_buggy_visibility_other("BEFORE UPDATE VISIBILITY","", &self.visibility);
    }
    pub fn update_visibility_after_move(&mut self, sx: usize, sy: usize, tx: usize, ty: usize, is_capture: bool) {
        // find_buggy_visibility_other("UPDATE VISIBILITY","BEFORE EVERYTHING", &self.visibility);
        // println!("({}, {}) -> ({}, {}) : {:?}", sx, sy, tx, ty, self.board[tx][ty]);
        match self.board[tx][ty] {
            Square::E => (), // how i implemented en passant
            Square::PW | Square::PB => self.visibility.update_pawn_visibility(tx, ty, &self.board),
            Square::NW | Square::NB => self.visibility.update_knight_visibility(tx, ty, &self.board),
            Square::BW | Square::BB => self.visibility.update_bishop_visibility(tx, ty, &self.board),
            Square::RW | Square::RB => self.visibility.update_rook_visibility(tx, ty, &self.board),
            Square::QW | Square::QB => self.visibility.update_queen_visibility(tx, ty, &self.board),
            Square::KW | Square::KB => self.visibility.update_king_visibility(tx, ty, &self.board)
        }
        // find_buggy_visibility_other("UPDATE VISIBILITY","AFTER FIRST", &self.visibility);
        self.visibility.update_after_blocking(&self.board, sx, sy);
        // find_buggy_visibility_other("UPDATE VISIBILITY","AFTER SECOND", &self.visibility);
        if !is_capture {
            self.visibility.update_after_blocking(&self.board, tx, ty);
        }
        // find_buggy_visibility_other("UPDATE VISIBILITY","AFTER LAST", &self.visibility);
        //self.piece_positions.move_piece(if self.board[tx][ty].is_white() {0} else {1}, sx, sy, tx, ty);
    }
    pub fn undo_update_visibility_after_move(&mut self, sx: usize, sy: usize, tx: usize, ty: usize) {
        if self.board[sx][sy].is_white() {
            self.visibility.clear_piece_from_existence(0, tx, ty);
        } else {
            self.visibility.clear_piece_from_existence(1, tx, ty);
        }
        match self.board[sx][sy] {
            Square::E => {
                println!("{}", self.to_fen());
                println!("total material: {}", self.evaluation.total_material);
                println!("castling: {:?}", self.king_moved);
                println!("kings: {:?}", self.kings);
                panic!("Undo Update Move on Empty Square");
            },
            Square::PW | Square::PB => self.visibility.update_pawn_visibility(sx, sy, &self.board),
            Square::NW | Square::NB => self.visibility.update_knight_visibility(sx, sy, &self.board),
            Square::BW | Square::BB => self.visibility.update_bishop_visibility(sx, sy, &self.board),
            Square::RW | Square::RB => self.visibility.update_rook_visibility(sx, sy, &self.board),
            Square::QW | Square::QB => self.visibility.update_queen_visibility(sx, sy, &self.board),
            Square::KW | Square::KB => self.visibility.update_king_visibility(sx, sy, &self.board)
        }
        // find_buggy_visibility_other("UNDO UPDATE VISIBILITY","AFTER FIRST", &self.visibility);
        match self.board[tx][ty] {
            Square::E => self.visibility.update_after_blocking(&self.board, tx, ty),
            Square::PW | Square::PB => self.visibility.update_pawn_visibility(tx, ty, &self.board),
            Square::NW | Square::NB => self.visibility.update_knight_visibility(tx, ty, &self.board),
            Square::BW | Square::BB => self.visibility.update_bishop_visibility(tx, ty, &self.board),
            Square::RW | Square::RB => self.visibility.update_rook_visibility(tx, ty, &self.board),
            Square::QW | Square::QB => self.visibility.update_queen_visibility(tx, ty, &self.board),
            Square::KW | Square::KB => self.visibility.update_king_visibility(tx, ty, &self.board)
        }
        // find_buggy_visibility_other("UNDO UPDATE VISIBILITY","AFTER SECOND", &self.visibility);
        self.visibility.update_after_blocking(&self.board, sx, sy);
        // find_buggy_visibility_other("UNDO UPDATE VISIBILITY","AFTER LAST", &self.visibility);
    }
    pub fn in_check(&self) -> bool {
        let kx;
        let ky;
        if self.white_to_move {
            kx = self.kings[0][0];
            ky = self.kings[0][1];
            self.visibility.map[kx][ky].total[1].len() > 0
        } else {
            kx = self.kings[1][0];
            ky = self.kings[1][1];
            self.visibility.map[kx][ky].total[0].len() > 0
        }
    }
    // big dicked king
    pub fn hung_king(&self) -> bool {
        let kx;
        let ky;
        if self.white_to_move {
            kx = self.kings[1][0];
            ky = self.kings[1][1];
            self.visibility.map[kx][ky].total[0].len() > 0
        } else {
            kx = self.kings[0][0];
            ky = self.kings[0][1];
            self.visibility.map[kx][ky].total[1].len() > 0
        }
    }
    pub fn inside_naked_king_endgame(&self) -> bool {
        let mut white_has_something = false;
        let mut black_has_something = false;
        for x in 0..8 {
            for y in 0..8 {
                match self.board[x][y] {
                    Square::E => continue,
                    Square::KB | Square::KW => continue,
                    piece if piece.is_white() => white_has_something = true,
                    _ => black_has_something = true
                }
            }
        }
        white_has_something != black_has_something
    }
    pub fn can_white_castle_kingside(&self) -> bool {
        if !self.board[7][5].is_empty() || !self.board[7][6].is_empty() {
            return false;
        }
        if can_attack_square(&self, 7, 5, 1)
            || can_attack_square(&self, 7, 6, 1) {
            return false;
        }
        self.board[7][7] == Square::RW
    }
    pub fn can_black_castle_kingside(&self) -> bool {
        if !self.board[0][5].is_empty() || !self.board[0][6].is_empty() {
            return false;
        }
        if can_attack_square(&self, 0, 5, 0)
            || can_attack_square(&self, 0, 6, 0) {
            return false;
        }
        self.board[0][7] == Square::RB
    }
    pub fn can_white_castle_queenside(&self) -> bool {
        if !self.board[7][3].is_empty() || !self.board[7][2].is_empty() || !self.board[7][1].is_empty() {
            return false;
        }
        if can_attack_square(&self, 7, 3, 1)
            || can_attack_square(&self, 7, 2, 1) {
            return false;
        }
        self.board[7][0] == Square::RW
    }
    pub fn can_black_castle_queenside(&self) -> bool {
        if !self.board[0][3].is_empty() || !self.board[0][2].is_empty() || !self.board[0][1].is_empty() {
            return false;
        }
        if can_attack_square(&self, 0, 3, 0)
            || can_attack_square(&self, 0, 2, 0) {
            return false;
        }
        self.board[0][0] == Square::RB
    }
    // TODO halfmove clock is not used
    pub fn make_move(&mut self, possible_move: &Move) {
        // overridden this in one branch don't want to make the code even uglier
        // println!("fen: {}, making move: {:?}", self.to_fen(), possible_move);
        match *possible_move {
            Move::EnPassant(a, b, c, d, _) => {
                // this is stupid not gonna fix it tho
                self.remove_eval_from(a, b);
                self.remove_eval_from(a, d);
                self.update_visibility_before_move(a, b, c, d);
                self.update_visibility_before_move(a, d, a, d);
                self.board[a][d] = Square::E;
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, false);
                self.update_visibility_after_move(a, d, a, d, true);
                self.add_eval_from(c, d);
            }
            // kingside castling
            Move::Castling(0, _, _, 6) => {
                self.remove_eval_from(0, 4) ;
                self.remove_eval_from(0, 7);
                self.update_visibility_before_move(0, 4, 0, 6);
                self.update_visibility_before_move(0, 7, 0, 5);
                self.board[0][6] = self.board[0][4];
                self.board[0][4] = Square::E;
                self.board[0][5] = self.board[0][7];
                self.board[0][7] = Square::E;
                self.king_moved[1] = self.clock;
                self.kings[1][1] = 6;
                self.update_visibility_after_move(0, 4, 0, 6, false);
                self.update_visibility_after_move(0, 7, 0, 5, false);
                self.add_eval_from(0, 6);
                self.add_eval_from(0, 5);
            }
            Move::Castling(_, _, _, 6) => {
                self.remove_eval_from(7, 4) ;
                self.remove_eval_from(7, 7);
                self.update_visibility_before_move(7, 4, 7, 6);
                self.update_visibility_before_move(7, 7, 7, 5);
                self.board[7][6] = self.board[7][4];
                self.board[7][4] = Square::E;
                self.board[7][5] = self.board[7][7];
                self.board[7][7] = Square::E;
                self.king_moved[0] = self.clock;
                self.kings[0][1] = 6;
                self.update_visibility_after_move(7, 4, 7, 6, false);
                self.update_visibility_after_move(7, 7, 7, 5, false);
                self.add_eval_from(7, 6);
                self.add_eval_from(7, 5);
            }
            // queenside castling
            Move::Castling(0, _, _, _) => {
                self.remove_eval_from(0, 4);
                self.remove_eval_from(0, 0);
                self.update_visibility_before_move(0, 4, 0, 2);
                self.update_visibility_before_move(0, 0, 0, 3);
                self.board[0][2] = self.board[0][4];
                self.board[0][4] = Square::E;
                self.board[0][3] = self.board[0][0];
                self.board[0][0] = Square::E;
                self.king_moved[1] = self.clock;
                self.kings[1][1] = 2;
                self.update_visibility_after_move(0, 4, 0, 2, false);
                self.update_visibility_after_move(0, 0, 0, 3, false);
                self.add_eval_from(0, 2);
                self.add_eval_from(0, 3);
            }
            Move::Castling(_, _, _, _) => {
                self.remove_eval_from(7, 4);
                self.remove_eval_from(7, 0);
                self.update_visibility_before_move(7, 4, 7, 2);
                self.update_visibility_before_move(7, 0, 7, 3);
                self.board[7][2] = self.board[7][4];
                self.board[7][4] = Square::E;
                self.board[7][3] = self.board[7][0];
                self.board[7][0] = Square::E;
                self.king_moved[0] = self.clock;
                self.kings[0][1] = 2;
                self.update_visibility_after_move(7, 4, 7, 2, false);
                self.update_visibility_after_move(7, 0, 7, 3, false);
                self.add_eval_from(7, 2);
                self.add_eval_from(7, 3);
            }
            Move::KingMove(0, 4, c, d, p) => {
                self.remove_eval_from(0, 4);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(0, 4, c, d);
                if self.king_moved[1] == 0 {
                    self.king_moved[1] = self.clock;
                }
                self.board[c][d] = self.board[0][4];
                self.board[0][4] = Square::E;
                self.kings[if self.white_to_move {0} else {1}] = [c, d];
                self.update_visibility_after_move(0, 4, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::KingMove(7, 4, c, d, p) => {
                self.remove_eval_from(7, 4);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(7, 4, c, d);
                if self.king_moved[0] == 0 {
                    self.king_moved[0] = self.clock;
                }
                self.board[c][d] = self.board[7][4];
                self.board[7][4] = Square::E;
                self.kings[if self.white_to_move {0} else {1}] = [c, d];
                self.update_visibility_after_move(7, 4, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::KingMove(a, b, c, d, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.kings[if self.white_to_move {0} else {1}] = [c, d];
                self.update_visibility_after_move(a, b, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::RookMove(a, b, c, d, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
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
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::TwoSquarePawnMove(1, b, _, _) => {
                self.remove_eval_from(1, b);
                self.update_visibility_before_move(1, b, 3, b);
                self.board[3][b] = self.board[1][b];
                self.board[1][b] = Square::E;
                self.en_passant_array[2][b] = self.clock + 1;
                self.en_passant_hash_index[self.clock + 1] = b;
                self.update_visibility_after_move(1, b, 3, b, false);
                self.add_eval_from(3, b);
            }
            Move::TwoSquarePawnMove(_, b, _, _) => {
                self.remove_eval_from(6, b);
                self.update_visibility_before_move(6, b, 4, b);
                self.board[4][b] = self.board[6][b];
                self.board[6][b] = Square::E;
                self.en_passant_array[5][b] = self.clock + 1;
                self.en_passant_hash_index[self.clock + 1] = b;
                self.update_visibility_after_move(6, b, 4, b, false);
                self.add_eval_from(4, b);
            }
            Move::PawnPromotion(a, b, c, d, cp, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = p;
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, !cp.is_empty());
                self.add_eval_from(c, d);
            }
            Move::PawnCapture(a, b, c, d, _) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, true);
                self.add_eval_from(c, d);
            }
            Move::PawnPush(a, b , c, d) => {
                self.remove_eval_from(a, b);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, false);
                self.add_eval_from(c, d);
            }
            Move::KnightMove(a, b, c, d, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::BishopMove(a, b, c, d, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
            Move::QueenMove(a, b, c, d, p) => {
                self.remove_eval_from(a, b);
                self.remove_eval_from(c, d);
                self.update_visibility_before_move(a, b, c, d);
                self.board[c][d] = self.board[a][b];
                self.board[a][b] = Square::E;
                self.update_visibility_after_move(a, b, c, d, !p.is_empty());
                self.add_eval_from(c, d);
            }
        }
        self.clock += 1;
        self.white_to_move = !self.white_to_move;
        // find_buggy_visibility("MOVE MADE", &format!("{:?}", possible_move), self);
        // println!("'{}' {:?} {}", self.to_fen(), possible_move, possible_move.to_string());
    }

    // TODO halfmove clock is not used
    pub fn undo_move(&mut self, previous_move: &Move) {
        // println!("fen: {}, undoing move: {:?}", self.to_fen(), previous_move);
        self.white_to_move = !self.white_to_move;
        self.clock -= 1;
        match *previous_move {
            Move::EnPassant(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = Square::E;
                self.board[a][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.undo_update_visibility_after_move(a, d, a, d);
                self.add_eval_from(a, b);
                self.add_eval_from(a, d);
            }
            // kingside castling
            Move::Castling(0, _, _, 6) => {
                self.remove_eval_from(0, 6);
                self.remove_eval_from(0, 5);
                self.board[0][4] = self.board[0][6];
                self.board[0][6] = Square::E;
                self.board[0][7] = self.board[0][5];
                self.board[0][5] = Square::E;
                self.king_moved[1] = 0;
                self.kings[1][1] = 4;
                self.undo_update_visibility_after_move(0, 4, 0, 6);
                self.undo_update_visibility_after_move(0, 7, 0, 5);
                self.add_eval_from(0, 4);
                self.add_eval_from(0, 7);
            }
            Move::Castling(_, _, _, 6) => {
                self.remove_eval_from(7, 6);
                self.remove_eval_from(7, 5);
                self.board[7][4] = self.board[7][6];
                self.board[7][6] = Square::E;
                self.board[7][7] = self.board[7][5];
                self.board[7][5] = Square::E;
                self.king_moved[0] = 0;
                self.kings[0][1] = 4;
                self.undo_update_visibility_after_move(7, 4, 7, 6);
                self.undo_update_visibility_after_move(7, 7, 7, 5);
                self.add_eval_from(7, 4);
                self.add_eval_from(7, 7);
            }
            // queenside castling
            Move::Castling(0, _, _, _) => {
                self.remove_eval_from(0, 2);
                self.remove_eval_from(0, 3);
                self.board[0][4] = self.board[0][2];
                self.board[0][2] = Square::E;
                self.board[0][0] = self.board[0][3];
                self.board[0][3] = Square::E;
                self.king_moved[1] = 0;
                self.kings[1][1] = 4;
                self.undo_update_visibility_after_move(0, 4, 0, 2);
                self.undo_update_visibility_after_move(0, 0, 0, 3);
                self.add_eval_from(0, 4);
                self.add_eval_from(0, 0);
            }
            Move::Castling(_, _, _, _) => {
                self.remove_eval_from(7, 2);
                self.remove_eval_from(7, 3);
                self.board[7][4] = self.board[7][2];
                self.board[7][2] = Square::E;
                self.board[7][0] = self.board[7][3];
                self.board[7][3] = Square::E;
                self.king_moved[0] = 0;
                self.kings[0][1] = 4;
                self.undo_update_visibility_after_move(7, 4, 7, 2);
                self.undo_update_visibility_after_move(7, 0, 7, 3);
                self.add_eval_from(7, 4);
                self.add_eval_from(7, 0);
            }
            Move::KingMove(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
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
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.kings[if self.white_to_move {0} else {1}] = [a, b];
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
            Move::RookMove(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
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
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
            Move::TwoSquarePawnMove(1, b, _, _) => {
                self.remove_eval_from(3, b);
                self.board[1][b] = self.board[3][b];
                self.board[3][b] = Square::E;
                self.en_passant_array[2][b] = 0;
                // 8 is the 'do nothing' code
                self.en_passant_hash_index[self.clock + 1] = 8;
                self.undo_update_visibility_after_move(1, b, 3, b);
                self.add_eval_from(1, b);
            }
            Move::TwoSquarePawnMove(_, b, _, _) => {
                self.remove_eval_from(4, b);
                self.board[6][b] = self.board[4][b];
                self.board[4][b] = Square::E;
                self.en_passant_array[5][b] = 0;
                self.en_passant_hash_index[self.clock + 1] = 8;
                self.undo_update_visibility_after_move(6, b, 4, b);
                self.add_eval_from(6, b);
            }
            Move::PawnPromotion(1, b, _, d, p, _) => {
                self.remove_eval_from(0, d);
                self.board[0][d] = p;
                self.board[1][b] = Square::PW;
                self.undo_update_visibility_after_move(1, b, 0, d);
                self.add_eval_from(1, b);
                self.add_eval_from(0, d);
            }
            Move::PawnPromotion(_, b, _, d, p, _) => {
                self.remove_eval_from(7, d);
                self.board[7][d] = p;
                self.board[6][b] = Square::PB;
                self.undo_update_visibility_after_move(6, b, 7, d);
                self.add_eval_from(6, b);
                self.add_eval_from(7, d);
            }
            Move::PawnCapture(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
            Move::PawnPush(a, b, c, d) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = Square::E;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
            }
            Move::KnightMove(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
            Move::BishopMove(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
            Move::QueenMove(a, b, c, d, p) => {
                self.remove_eval_from(c, d);
                self.board[a][b] = self.board[c][d];
                self.board[c][d] = p;
                self.undo_update_visibility_after_move(a, b, c, d);
                self.add_eval_from(a, b);
                self.add_eval_from(c, d);
            }
        }
        // find_buggy_visibility("MOVE UNDONE", &format!("{:?}", previous_move), self);
    }

    // don't always need to add and subtract but idc
    pub fn remove_eval_from(&mut self, x: usize, y: usize) {
        let piece = self.board[x][y] as usize;
        self.evaluation.total_material -= self.board[x][y].worth();
        self.evaluation.material -= MATERIAL_WORTH[piece];
        self.evaluation.positioning -= PIECE_SQUARE_TABLE[piece][x][y];
    }

    pub fn add_eval_from(&mut self, x: usize, y: usize) {
        let piece = self.board[x][y] as usize;
        self.evaluation.total_material += self.board[x][y].worth();
        self.evaluation.material += MATERIAL_WORTH[piece];
        self.evaluation.positioning += PIECE_SQUARE_TABLE[piece][x][y];
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
        println!("en_passant_hash_index: {:?}", self.en_passant_hash_index);
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
                    Square::PW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('P')},
                    Square::PB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('p')},
                    Square::NW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('N')},
                    Square::NB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('n')},
                    Square::BW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('B')},
                    Square::BB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('b')},
                    Square::RW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('R')},
                    Square::RB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('r')},
                    Square::QW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('Q')},
                    Square::QB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('q')},
                    Square::KW => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('K')},
                    Square::KB => {if counter != 0 {string.push(char::from_digit(counter, 10).unwrap()); counter = 0} string.push('k')}
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


#[derive(Debug)]
pub struct SquareVisibility {
    // pub pawns: [usize; 2],
    pub total: [Vec<(usize, usize)>; 2]
}

impl SquareVisibility {
    pub fn new() -> SquareVisibility {
        SquareVisibility {
            // pawns: [0, 0],
            total: [Vec::with_capacity(50), Vec::with_capacity(50)]
        }
    }
}


#[derive(Debug)]
pub struct Evaluation {
    // white pieces attacking black king
    // black pieces attacking white king
    pub total_material: i64,
    pub eyed_values: i64,
    // total will not be modified by the values above
    // a new value will be copied and returned separately
    pub material: i64,
    pub positioning: i64
}

impl Evaluation {
    pub fn default() -> Evaluation {
        Evaluation {
            total_material: 78,
            eyed_values: 0,
            material: 0,
            positioning: 0
        }
    }
    pub fn initialized() -> Evaluation {
        Evaluation {
            total_material: 0,
            eyed_values: 0,
            material: 0,
            positioning: 0
        }
    }
    pub fn new(inner_board: &[[Square; 8]; 8]) -> Evaluation {
        let mut output = Evaluation::initialized();
        let mut piece;
        for x in 0..8 {
            for y in 0..8 {
                piece = inner_board[x][y] as usize;
                if inner_board[x][y] != Square::KW && inner_board[x][y] != Square::KB {
                    output.total_material += inner_board[x][y].worth();
                }
                output.material += MATERIAL_WORTH[piece];
                output.positioning += PIECE_SQUARE_TABLE[piece][x][y];
            }
        }
        output
    }
}

#[derive(Debug)]
pub struct Visibility {
    // if a square is seen from another square
    pub from_to: [[[[bool; 8]; 8]; 8]; 8],
    // where are the pieces that see the square
    pub map: [[SquareVisibility; 8]; 8],
    // what squares can a piece see
    pub pieces: [[Vec<(usize, usize)>; 8]; 8]
}

impl Visibility {
    // pub fn clear_pawn_from_existence(&mut self, white_black: usize, x: usize, y: usize) {
    //     for (cx, cy) in &self.pieces[x][y] {
    //         self.map[*cx][*cy].pawns[white_black] -= 1;
    //         remove_from_map(&mut self.map[*cx][*cy], (x, y), white_black);
    //     }
    //     self.pieces[x][y].clear();
    // }
    pub fn clear_piece_from_existence(&mut self, white_black: usize, x: usize, y: usize) {
        for (cx, cy) in &self.pieces[x][y] {
            self.from_to[x][y][*cx][*cy] = false;
            remove_from_map(&mut self.map[*cx][*cy], (x, y), white_black);
        }
        self.pieces[x][y].clear();
    }
    pub fn update_knight_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
            if let Some((cx, cy)) = *i {
                self.map[cx][cy].total[white_black].push((x, y));
                self.from_to[x][y][cx][cy] = true;
                self.pieces[x][y].push((cx, cy));
            } else {
                break;
            }
        }
    }
    pub fn update_bishop_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
            for j in i {
                if let Some((cx, cy)) = *j {
                    self.from_to[x][y][cx][cy] = true;
                    self.map[cx][cy].total[white_black].push((x, y));
                    self.pieces[x][y].push((cx, cy));
                    if !inner_board[cx][cy].is_empty() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    pub fn update_rook_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
            for j in i {
                if let Some((cx, cy)) = *j {
                    self.from_to[x][y][cx][cy] = true;
                    self.map[cx][cy].total[white_black].push((x, y));
                    self.pieces[x][y].push((cx, cy));
                    if !inner_board[cx][cy].is_empty() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    pub fn update_queen_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        self.update_bishop_visibility(x, y, inner_board);
        self.update_rook_visibility(x, y, inner_board);
    }
    pub fn update_king_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
            if let Some((cx, cy)) = i[0] {
                self.from_to[x][y][cx][cy] = true;
                self.map[cx][cy].total[white_black].push((x, y));
                self.pieces[x][y].push((cx, cy));
            }
        }
        for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
            if let Some((cx, cy)) = i[0] {
                self.from_to[x][y][cx][cy] = true;
                self.map[cx][cy].total[white_black].push((x, y));
                self.pieces[x][y].push((cx, cy));
            }
        }
    }
    // pawns are cross eyed do not see in front of them
    pub fn update_pawn_visibility(&mut self, x: usize, y: usize, inner_board: &[[Square; 8]; 8]) {
        let cx;
        let white_black;
        if inner_board[x][y].is_white() {
            white_black = 0;
            cx = x - 1;
        } else {
            white_black = 1;
            cx = x + 1;
        }
        if y > 0 {
            // self.map[cx][y - 1].pawns[white_black] += 1;
            // r2q2nr/1pp1k1pp/2n2p2/pB1p4/1b1P2b1/2N1PN2/PPPB1PPP/R2QK2R w KQ - 3 9 239732 234055
            self.map[cx][y - 1].total[white_black].push((x, y));
            self.from_to[x][y][cx][y - 1] = true;
            self.pieces[x][y].push((cx, y - 1));
        }
        if y < 7 {
            // self.map[cx][y + 1].pawns[white_black] += 1;
            // assert!(!self.from_to[x][y][cx][y + 1]);
            // 6k1/1pp3p1/4p3/3r2p1/6P1/p7/4K3/8 b - - 1 41 150058 85082
            self.map[cx][y + 1].total[white_black].push((x, y));
            self.from_to[x][y][cx][y + 1] = true;
            self.pieces[x][y].push((cx, y + 1));
        }
    }
    pub fn initialize() -> Visibility {
        let from_to = [[[[false; 8]; 8]; 8]; 8];
        let map = [(); 8].map(|_| [(); 8].map(|_| SquareVisibility::new()));
        let pieces = [(); 8].map(|_| [(); 8].map(|_| Vec::new()));
        Visibility {
            from_to,
            map,
            pieces
        }
    }
    pub fn new(inner_board: &[[Square; 8]; 8]) -> Visibility {
        let mut output = Visibility::initialize();
        for x in 0..8 {
            for y in 0..8 {
                match inner_board[x][y] {
                    Square::E => continue,
                    Square::PW | Square::PB => output.update_pawn_visibility(x, y, inner_board),
                    Square::NW | Square::NB => output.update_knight_visibility(x, y, inner_board),
                    Square::BW | Square::BB => output.update_bishop_visibility(x, y, inner_board),
                    Square::RW | Square::RB => output.update_rook_visibility(x, y, inner_board),
                    Square::QW | Square::QB => output.update_queen_visibility(x, y, inner_board),
                    Square::KW | Square::KB => output.update_king_visibility(x, y, inner_board)
                }
            }
        }
        output
    }
    pub fn update_after_blocking(&mut self, inner_board: &[[Square; 8]; 8], x: usize, y: usize) {
        for i in 0..4 {
            for j in PRECOMPUTED_BISHOP_MOVES[x][y][i].iter() {
                if let Some((cx, cy)) = *j {
                    match inner_board[cx][cy] {
                        Square::E => continue,
                        Square::BW | Square::BB | Square::QW | Square::QB => {
                            self.update_bishop_diagonal(inner_board, [3, 2, 1, 0][i], cx, cy);
                            break;
                        },
                        _ => { break }
                    }
                }
            }
        }
        for i in 0..4 {
            for j in PRECOMPUTED_ROOK_MOVES[x][y][i].iter() {
                if let Some((cx, cy)) = *j {
                    match inner_board[cx][cy] {
                        Square::E => continue,
                        Square::RW | Square::RB | Square::QW | Square::QB => {
                            self.update_rook_diagonal(inner_board, [1, 0, 3, 2][i], cx, cy);
                            break;
                        },
                        _ => { break }
                    }
                }
            }
        }
    }
    fn update_bishop_diagonal(&mut self, inner_board: &[[Square; 8]; 8], diagonal_index: usize, x: usize, y: usize) {
        let mut index = 0;
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        while index < 7 {
            if let Some((cx, cy)) = PRECOMPUTED_BISHOP_MOVES[x][y][diagonal_index][index] {
                if !self.from_to[x][y][cx][cy] {
                    self.map[cx][cy].total[white_black].push((x, y));
                    self.pieces[x][y].push((cx, cy));
                    self.from_to[x][y][cx][cy] = true;
                }
                index += 1;
                if !inner_board[cx][cy].is_empty() {
                    break;
                }
            } else {
                return;
            }
        }
        while index < 7 {
            if let Some((cx, cy)) = PRECOMPUTED_BISHOP_MOVES[x][y][diagonal_index][index] {
                if !self.from_to[x][y][cx][cy] {
                    return;
                }
                self.from_to[x][y][cx][cy] = false;
                remove_from_piece(&mut self.pieces[x][y], (cx, cy));
                remove_from_map(&mut self.map[cx][cy], (x, y), white_black);
                index += 1;
            } else {
                break;
            }
        }
    }
    fn update_rook_diagonal(&mut self, inner_board: &[[Square; 8]; 8], diagonal_index: usize, x: usize, y: usize) {
        let mut index = 0;
        let white_black = if inner_board[x][y].is_white() {0} else {1};
        while index < 7 {
            if let Some((cx, cy)) = PRECOMPUTED_ROOK_MOVES[x][y][diagonal_index][index] {
                if !self.from_to[x][y][cx][cy] {
                    self.from_to[x][y][cx][cy] = true;
                    self.map[cx][cy].total[white_black].push((x, y));
                    self.pieces[x][y].push((cx, cy));
                }
                index += 1;
                if !inner_board[cx][cy].is_empty() {
                    break;
                }
            } else {
                return;
            }
        }
        while index < 7 {
            if let Some((cx, cy)) = PRECOMPUTED_ROOK_MOVES[x][y][diagonal_index][index] {
                if !self.from_to[x][y][cx][cy] {
                    return;
                }
                self.from_to[x][y][cx][cy] = false;
                remove_from_piece(&mut self.pieces[x][y], (cx, cy));
                remove_from_map(&mut self.map[cx][cy], (x, y), white_black);
                index += 1;
            } else {
                break;
            }
        }
    }
}

// TODO write a separate function for non-captures
pub fn generate_into(board: &Board, vector: &mut Vec<Move>) {
    if board.white_to_move {
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::PW => {
                        generate_pawn_moves(board, x, y, vector);
                    },
                    Square::NW => {
                        generate_knight_moves(board, x, y, vector);
                    },
                    Square::BW => {
                        generate_bishop_moves(board, x, y, vector);
                    },
                    Square::RW => {
                        generate_rook_moves(board, x, y, vector);
                    },
                    Square::QW => {
                        generate_queen_moves(board, x, y, vector);
                    },
                    Square::KW => {
                        generate_king_moves(board, x, y, vector);
                    },
                    _ => ()
                }
            }
        }
    } else {
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::PB => {
                        generate_pawn_moves(board, x, y, vector);
                    },
                    Square::NB => {
                        generate_knight_moves(board, x, y, vector);
                    },
                    Square::BB => {
                        generate_bishop_moves(board, x, y, vector);
                    },
                    Square::RB => {
                        generate_rook_moves(board, x, y, vector);
                    },
                    Square::QB => {
                        generate_queen_moves(board, x, y, vector);
                    },
                    Square::KB => {
                        generate_king_moves(board, x, y, vector);
                    },
                    _ => ()
                }
            }
        }
    }
}

pub fn generate_captures_into(board: &Board, vector: &mut Vec<Move>) {
    if board.white_to_move {
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::PW => {
                        generate_pawn_captures(board, x, y, vector);
                    },
                    Square::NW => {
                        generate_knight_captures(board, x, y, vector);
                    },
                    Square::BW => {
                        generate_bishop_captures(board, x, y, vector);
                    },
                    Square::RW => {
                        generate_rook_captures(board, x, y, vector);
                    },
                    Square::QW => {
                        generate_queen_captures(board, x, y, vector);
                    },
                    Square::KW => {
                        generate_king_captures(board, x, y, vector);
                    },
                    _ => ()
                }
            }
        }
    } else {
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::PB => {
                        generate_pawn_captures(board, x, y, vector);
                    },
                    Square::NB => {
                        generate_knight_captures(board, x, y, vector);
                    },
                    Square::BB => {
                        generate_bishop_captures(board, x, y, vector);
                    },
                    Square::RB => {
                        generate_rook_captures(board, x, y, vector);
                    },
                    Square::QB => {
                        generate_queen_captures(board, x, y, vector);
                    },
                    Square::KB => {
                        generate_king_captures(board, x, y, vector);
                    },
                    _ => ()
                }
            }
        }
    }
}

pub fn generate_checks_into(board: &Board, vector: &mut Vec<Move>) {
    if board.white_to_move {
        let kx = board.kings[1][0];
        let ky = board.kings[1][1];
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::NW => {
                        generate_knight_checks(board, kx, ky, x, y, vector);
                    },
                    Square::BW => {
                        generate_bishop_checks(board, kx, ky, x, y, vector)
                    },
                    Square::RW => {
                        generate_rook_checks(board, kx, ky, x, y, vector)
                    },
                    Square::QW => {
                        generate_queen_checks(board, kx, ky, x, y, vector)
                    },
                    _ => continue
                }
            }
        }
        if kx > 4 {
            return;
        }
        // fuck efficiency I don't want ugly ass inbred looking code
        if ky > 0 && board.board[kx + 1][ky - 1].is_empty() {
            if board.board[kx + 2][ky - 1] == Square::PW {
                vector.push(Move::PawnPush(kx + 2,
                                           ky - 1,
                                           kx + 1,
                                           ky - 1));
            } else if kx == 3 && board.board[6][ky - 1] == Square::PW && board.board[5][ky - 1].is_empty() {
                vector.push(Move::TwoSquarePawnMove(6, ky - 1, 4, ky - 1));
            }
        }
        if ky < 7 && board.board[kx + 1][ky + 1].is_empty() {
            if board.board[kx + 2][ky + 1] == Square::PW {
                vector.push(Move::PawnPush(kx + 2,
                                           ky + 1,
                                           kx + 1,
                                           ky + 1));
            } else if kx == 3 && board.board[6][ky + 1] == Square::PW && board.board[5][ky + 1].is_empty() {
                vector.push(Move::TwoSquarePawnMove(6, ky + 1, 4, ky + 1));
            }
        }
    } else {
        let kx = board.kings[0][0];
        let ky = board.kings[0][1];
        for x in 0..8 {
            for y in 0..8 {
                match board.board[x][y] {
                    Square::NB => {
                        generate_knight_checks(board, kx, ky, x, y, vector);
                    },
                    Square::BB => {
                        generate_bishop_checks(board, kx, ky, x, y, vector)
                    },
                    Square::RB => {
                        generate_rook_checks(board, kx, ky, x, y, vector)
                    },
                    Square::QB => {
                        generate_queen_checks(board, kx, ky, x, y, vector)
                    },
                    _ => continue
                }
            }
        }
        if kx < 3 {
            return;
        }
        if ky > 0 && board.board[kx - 1][ky - 1].is_empty() {
            if board.board[kx - 2][ky - 1] == Square::PB {
                vector.push(Move::PawnPush(kx - 2,
                                           ky - 1,
                                           kx - 1,
                                           ky - 1));
            } else if kx == 4 && board.board[1][ky - 1] == Square::PB && board.board[2][ky - 1].is_empty() {
                vector.push(Move::TwoSquarePawnMove(1, ky - 1, 3, ky - 1));
            }
        }
        if ky < 7 && board.board[kx - 1][ky + 1].is_empty() {
            if board.board[kx - 2][ky + 1] == Square::PB {
                vector.push(Move::PawnPush(kx - 2,
                                           ky + 1,
                                           kx - 1,
                                           ky + 1));
            } else if kx == 4 && board.board[1][ky + 1] == Square::PB && board.board[2][ky + 1].is_empty() {
                vector.push(Move::TwoSquarePawnMove(1, ky + 1, 3, ky + 1));
            }
        }
    }
}

pub fn generate_check_blocks(board: &Board, kx: usize, ky: usize, px: usize, py: usize, vector: &mut Vec<Move>) {
    if kx.abs_diff(px) < 2 && ky.abs_diff(py) < 2 {
        return;
    }
    match board.board[px][py] {
        Square::PW | Square::PB => (),
        Square::NW | Square::NB => (),
        Square::BW | Square::BB => {
            let diagonal = match (px > kx, py > ky) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3
            };
            for i in PRECOMPUTED_BISHOP_MOVES[kx][ky][diagonal].iter() {
                if let Some((x, y)) = *i {
                    if x == px {
                        break;
                    }
                    generate_blockers_into(board, vector, x, y);
                } else {
                    break;
                }
            }
        },
        Square::RW | Square::RB => {
            if px == kx {
                let diagonal = if py > ky {2} else {3};
                for i in PRECOMPUTED_ROOK_MOVES[kx][ky][diagonal].iter() {
                    if let Some((x, y)) = *i {
                        if y == py {
                            break;
                        }
                        generate_blockers_into(board, vector, x, y);
                    } else {
                        break;
                    }
                }
            } else {
                let diagonal = if px > kx {0} else {1};
                for i in PRECOMPUTED_ROOK_MOVES[kx][ky][diagonal].iter() {
                    if let Some((x, y)) = *i {
                        if x == px {
                            break;
                        }
                        generate_blockers_into(board, vector, x, y);
                    } else {
                        break;
                    }
                }
            }
        },
        Square::QW | Square::QB => {
            if px == kx {
                let diagonal = if py > ky {2} else {3};
                for i in PRECOMPUTED_ROOK_MOVES[kx][ky][diagonal].iter() {
                    if let Some((x, y)) = *i {
                        if y == py {
                            break;
                        }
                        generate_blockers_into(board, vector, x, y);
                    } else {
                        break;
                    }
                }
            } else if py == ky {
                let diagonal = if px > kx {0} else {1};
                for i in PRECOMPUTED_ROOK_MOVES[kx][ky][diagonal].iter() {
                    if let Some((x, y)) = *i {
                        if x == px {
                            break;
                        }
                        generate_blockers_into(board, vector, x, y);
                    } else {
                        break;
                    }
                }
            } else {
                let diagonal = match (px > kx, py > ky) {
                    (true, true) => 0,
                    (true, false) => 1,
                    (false, true) => 2,
                    (false, false) => 3
                };
                for i in PRECOMPUTED_BISHOP_MOVES[kx][ky][diagonal].iter() {
                    if let Some((x, y)) = *i {
                        if x == px {
                            break;
                        }
                        generate_blockers_into(board, vector, x, y);
                    } else {
                        break;
                    }
                }
            }
        },
        _ => ()
    }
}

pub fn generate_check_evasions_into(board: &Board, vector: &mut Vec<Move>) {
    // emits captures
    let (white_black, black_white) = if board.white_to_move {(0, 1)} else {(1, 0)};
    let kx = board.kings[white_black][0];
    let ky = board.kings[white_black][1];
    match board.visibility.map[kx][ky].total[black_white].len() {
        0 => return,
        1 => {
            generate_check_blocks(
                board,
                kx,
                ky,
                board.visibility.map[kx][ky].total[black_white][0].0,
                board.visibility.map[kx][ky].total[black_white][0].1,
                vector
            );
            generate_king_moves(board, kx, ky, vector);
        },
        _ => {
            generate_king_moves(board, kx, ky, vector);
        }
    }
}
pub fn generate_check_counters_into(board: &Board, vector: &mut Vec<Move>) {
    // emits captures
    let (white_black, black_white) = if board.white_to_move {(0, 1)} else {(1, 0)};
    let kx = board.kings[white_black][0];
    let ky = board.kings[white_black][1];
    match board.visibility.map[kx][ky].total[black_white].len() {
        0 => return,
        1 => {
            generate_defenders_into(
                board,
                vector,
                board.visibility.map[kx][ky].total[black_white][0].0,
                board.visibility.map[kx][ky].total[black_white][0].1
            );
            generate_check_blocks(
                board,
                kx,
                ky,
                board.visibility.map[kx][ky].total[black_white][0].0,
                board.visibility.map[kx][ky].total[black_white][0].1,
                vector
            );
            generate_king_moves(board, kx, ky, vector);
        },
        _ => {
            generate_king_moves(board, kx, ky, vector);
        }
    }
}

pub fn remove_from_piece(vector: &mut Vec<(usize, usize)>, item: (usize, usize)) {
    let mut index = 0;
    loop {
        if index >= vector.len() {
            return
        } else if vector[index] == item {
            break;
        } else {
            index += 1;
        }
    }
    vector.swap_remove(index);
}

pub fn remove_from_map(square_visibility: &mut SquareVisibility, item: (usize, usize), white_black: usize) {
    // fuck dem pawns
    let mut index = 0;
    loop {
        if index >= square_visibility.total[white_black].len() {
            return;
        } else if square_visibility.total[white_black][index] == item {
            break;
        } else {
            index += 1;
        }
    }
    // println!("{:?}", square_visibility.total[white_black]);
    square_visibility.total[white_black].swap_remove(index);
    // for i in square_visibility.total[white_black].iter() {
    //     assert_ne!(*i, item);
    // }
}

pub fn maybe_insert(vector: &mut Vec<(usize, usize)>, item: (usize, usize)) -> bool {
    for i in vector.iter() {
        if *i == item {
            return false;
        }
    }
    vector.push(item);
    true
}

// ignore the name this will also get non-captures
pub fn get_capture(board: &Board, hash: usize) -> Option<Move> {
    let sx = hash >> 9;
    let sy = (hash >> 6) & 7;
    let tx = (hash >> 3) & 7;
    let ty = hash & 7;
    match board.board[sx][sy] {
        Square::E => None,
        Square::PW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(true) {
                    if sx == 1 && tx == 0 {
                        Some(Move::PawnPromotion(1, sy, 0, ty, board.board[tx][ty], Square::QW))
                    } else {
                        Some(Move::PawnCapture(sx, sy, tx, ty, board.board[tx][ty]))
                    }
                } else if board.en_passant_array[tx][ty] == board.clock {
                    Some(Move::EnPassant(sx, sy, tx, ty, Square::PB))
                } else {
                    None
                }
            } else if sy == ty && board.board[tx][ty].is_empty() {
                if sx == 1 && tx == 0 {
                    // don't have enough information it's prob a queen tho
                    Some(Move::PawnPromotion(1, sy, 0, ty, Square::E, Square::QW))
                } else if sx == tx + 1 {
                    Some(Move::PawnPush(sx, sy, tx, ty))
                } else if sx == 6 && tx == 4 && board.board[5][ty].is_empty() {
                    Some(Move::TwoSquarePawnMove(6, ty, 4, ty))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Square::PB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(false) {
                    if sx == 6 && tx == 7 {
                        Some(Move::PawnPromotion(6, sy, 7, ty, board.board[tx][ty], Square::QB))
                    } else {
                        Some(Move::PawnCapture(sx, sy, tx, ty, board.board[tx][ty]))
                    }
                } else if board.en_passant_array[tx][ty] == board.clock {
                    Some(Move::EnPassant(sx, sy, tx, ty, Square::PW))
                } else {
                    None
                }
            } else if sy == ty && board.board[tx][ty].is_empty() {
                if sx == 6 && tx == 7 {
                    // don't have enough information it's prob a queen tho
                    Some(Move::PawnPromotion(6, sy, 7, ty, Square::E, Square::QB))
                } else if sx + 1 == tx {
                    Some(Move::PawnPush(sx, sy, tx, ty))
                } else if sx == 1 && tx == 3 && board.board[2][ty].is_empty() {
                    Some(Move::TwoSquarePawnMove(1, ty, 3, ty))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Square::NW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::KnightMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::NB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::KnightMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::BW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::BishopMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::BB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::BishopMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::RW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::RookMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::RB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::RookMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::QW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::QueenMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::QB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::QueenMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::KW => {
            if !board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(true) && board.visibility.map[tx][ty].total[1].len() > 0 {
                    Some(Move::KingMove(sx, sy, tx, ty, board.board[tx][ty]))
                } else {
                    None
                }
            } else if board.king_moved[0] == 0 {
                match (tx, ty) {
                    (7, 6) => {
                        if board.rook_moved[0] == 0 && !board.in_check() && board.can_white_castle_kingside() {
                            Some(Move::Castling(7, 4, 7, 6))
                        } else {
                            None
                        }
                    },
                    (7, 2) => {
                        if board.rook_moved[1] == 0 && !board.in_check() && board.can_white_castle_queenside()  {
                            Some(Move::Castling(7, 4, 7, 2))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            } else {
                None
            }
        },
        Square::KB => {
            if board.white_to_move {
                return None
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(false) && board.visibility.map[tx][ty].total[0].len() > 0 {
                    Some(Move::KingMove(sx, sy, tx, ty, board.board[tx][ty]))
                } else {
                    None
                }
            } else if board.king_moved[1] == 0 {
                match (tx, ty) {
                    (0, 6) => {
                        if board.rook_moved[2] == 0 && !board.in_check() && board.can_black_castle_kingside() {
                            Some(Move::Castling(0, 4, 0, 6))
                        } else {
                            None
                        }
                    },
                    (0, 2) => {
                        if board.rook_moved[3] == 0 && !board.in_check() && board.can_black_castle_queenside() {
                            Some(Move::Castling(0, 4, 0, 2))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            } else {
                None
            }
        }
    }
}

// will also return captures main difference will be that i save some ifs
// move stored is not a capture (fucks with pawns)
pub fn get_move(board: &Board, hash: usize) -> Option<Move> {
    let sx = hash >> 9;
    let sy = (hash >> 6) & 7;
    let tx = (hash >> 3) & 7;
    let ty = hash & 7;
    match board.board[sx][sy] {
        Square::E => None,
        Square::PW => {
            if !board.white_to_move || tx == 0 {
                return None;
            }
            // no captures grrrr
            if sy == ty && board.board[tx][ty].is_empty() {
                if sx == tx + 1 {
                    Some(Move::PawnPush(sx, sy, tx, ty))
                } else if sx == 6 && tx == 4 && board.board[5][ty].is_empty() {
                    Some(Move::TwoSquarePawnMove(6, ty, 4, ty))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Square::PB => {
            if board.white_to_move || tx == 7 {
                return None;
            }
            if sy == ty && board.board[tx][ty].is_empty() {
                if sx + 1 == tx {
                    Some(Move::PawnPush(sx, sy, tx, ty))
                } else if sx == 1 && tx == 3 && board.board[2][ty].is_empty() {
                    Some(Move::PawnPush(1, ty, 3, ty))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Square::NW => {
            if !board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::KnightMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::NB => {
            if board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::KnightMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::BW => {
            if !board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::BishopMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::BB => {
            if board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::BishopMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::RW => {
            if !board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::RookMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::RB => {
            if board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::RookMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::QW => {
            if !board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(true) {
                Some(Move::QueenMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::QB => {
            if board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] && board.board[tx][ty].can_capture(false) {
                Some(Move::QueenMove(sx, sy, tx, ty, board.board[tx][ty]))
            } else {
                None
            }
        },
        Square::KW => {
            if !board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(true) && board.visibility.map[tx][ty].total[1].len() > 0 {
                    Some(Move::KingMove(sx, sy, tx, ty, board.board[tx][ty]))
                } else {
                    None
                }
            } else if board.king_moved[0] == 0 {
                match (tx, ty) {
                    (7, 6) => {
                        if board.rook_moved[0] == 0 && !board.in_check() && board.can_white_castle_kingside() {
                            Some(Move::Castling(7, 4, 7, 6))
                        } else {
                            None
                        }
                    },
                    (7, 2) => {
                        if board.rook_moved[1] == 0 && !board.in_check() && board.can_white_castle_queenside() {
                            Some(Move::Castling(7, 4, 7, 2))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            } else {
                None
            }
        },
        Square::KB => {
            if board.white_to_move {
                return None;
            }
            if board.visibility.from_to[sx][sy][tx][ty] {
                if board.board[tx][ty].can_capture(false) && board.visibility.map[tx][ty].total[0].len() > 0 {
                    Some(Move::KingMove(sx, sy, tx, ty, board.board[tx][ty]))
                } else {
                    None
                }
            } else if board.king_moved[1] == 0 {
                match (tx, ty) {
                    (0, 6) => {
                        if board.rook_moved[2] == 0 && !board.in_check() && board.can_black_castle_kingside() {
                            Some(Move::Castling(0, 4, 0, 6))
                        } else {
                            None
                        }
                    },
                    (0, 2) => {
                        if board.rook_moved[3] == 0 && !board.in_check() && board.can_black_castle_queenside() {
                            Some(Move::Castling(0, 4, 0, 2))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            } else {
                None
            }
        }
    }
}

// purposefully don't clear kill_moves
pub fn clear_cache(cache: &mut HashMapReuser<u64, i64>,
                   fuck_moves: &mut Vec<BestMoves>,
                   marry_moves: &mut [i64; 896]) {
    for i in cache.hashmaps.iter_mut() {
        i.clear();
    }
    // for i in fuck_moves.iter_mut() {
    //     i.moves[0] = 4096;
    //     i.moves[1] = 4096;
    //     i.moves[2] = 4096;
    // }
    // marry_moves.fill(0);
}
