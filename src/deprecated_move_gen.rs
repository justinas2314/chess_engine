use crate::utils::{Board, Move, Square};

pub struct KnightMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    counter: usize,
    white: bool,
    data: [(isize, isize); 8],
}

impl KnightMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> KnightMoveGenerator<'_> {
        let xx = x as isize;
        let yy = y as isize;
        KnightMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            counter: 0,
            data: [
                (xx - 2, yy - 1),
                (xx - 2, yy + 1),
                (xx + 2, yy - 1),
                (xx + 2, yy + 1),
                (xx - 1, yy - 2),
                (xx - 1, yy + 2),
                (xx + 1, yy - 2),
                (xx + 1, yy + 2)
            ],
        }
    }
}


impl Iterator for KnightMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x;
        let mut y;
        loop {
            match self.data.get(self.counter) {
                None => return None,
                Some((xx @ 0..=7, yy @ 0..=7)) => {
                    x = *xx as usize;
                    y = *yy as usize;
                    if self.board.board[x][y].can_move_to(self.white) {
                        self.counter += 1;
                        return Some(Move::KnightMove(self.x, self.y, x, y, self.board.board[x][y]));
                    } else {
                        self.counter += 1;
                    }
                }
                _ => {
                    self.counter += 1;
                }
            };
        }
    }
}


pub struct BishopMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    first_counter: usize,
    second_counter: usize,
    white: bool,
    data: [[(isize, isize); 7]; 4],
}


impl BishopMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> BishopMoveGenerator {
        let xx = x as isize;
        let yy = y as isize;
        BishopMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            first_counter: 0,
            second_counter: 0,
            data: [
                [
                    (xx + 1, yy + 1),
                    (xx + 2, yy + 2),
                    (xx + 3, yy + 3),
                    (xx + 4, yy + 4),
                    (xx + 5, yy + 5),
                    (xx + 6, yy + 6),
                    (xx + 7, yy + 7)
                ],
                [
                    (xx + 1, yy - 1),
                    (xx + 2, yy - 2),
                    (xx + 3, yy - 3),
                    (xx + 4, yy - 4),
                    (xx + 5, yy - 5),
                    (xx + 6, yy - 6),
                    (xx + 7, yy - 7)
                ],
                [
                    (xx - 1, yy + 1),
                    (xx - 2, yy + 2),
                    (xx - 3, yy + 3),
                    (xx - 4, yy + 4),
                    (xx - 5, yy + 5),
                    (xx - 6, yy + 6),
                    (xx - 7, yy + 7)
                ],
                [
                    (xx - 1, yy - 1),
                    (xx - 2, yy - 2),
                    (xx - 3, yy - 3),
                    (xx - 4, yy - 4),
                    (xx - 5, yy - 5),
                    (xx - 6, yy - 6),
                    (xx - 7, yy - 7)
                ]
            ],
        }
    }
}


impl Iterator for BishopMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x;
        let mut y;
        loop {
            if let Some(arr) = self.data.get(self.first_counter) {
                match arr.get(self.second_counter) {
                    Some((xx @ 0..=7, yy @ 0..=7)) => {
                        x = *xx as usize;
                        y = *yy as usize;
                        match self.board.board[x][y].can_move_to_and_through(self.white) {
                            (true, true) => {
                                self.second_counter += 1;
                                return Some(Move::BishopMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            (true, false) => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                                return Some(Move::BishopMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            _ => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                            }
                        }
                    }
                    _ => {
                        self.first_counter += 1;
                        self.second_counter = 0
                    }
                }
            } else {
                return None;
            }
        }
    }
}


pub struct RookMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    first_counter: usize,
    second_counter: usize,
    white: bool,
    data: [[(isize, isize); 7]; 4],
}


impl RookMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> RookMoveGenerator {
        let xx = x as isize;
        let yy = y as isize;
        RookMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            first_counter: 0,
            second_counter: 0,
            data: [
                [
                    (xx, yy + 1),
                    (xx, yy + 2),
                    (xx, yy + 3),
                    (xx, yy + 4),
                    (xx, yy + 5),
                    (xx, yy + 6),
                    (xx, yy + 7)
                ],
                [
                    (xx, yy - 1),
                    (xx, yy - 2),
                    (xx, yy - 3),
                    (xx, yy - 4),
                    (xx, yy - 5),
                    (xx, yy - 6),
                    (xx, yy - 7)
                ],
                [
                    (xx + 1, yy),
                    (xx + 2, yy),
                    (xx + 3, yy),
                    (xx + 4, yy),
                    (xx + 5, yy),
                    (xx + 6, yy),
                    (xx + 7, yy)
                ],
                [
                    (xx - 1, yy),
                    (xx - 2, yy),
                    (xx - 3, yy),
                    (xx - 4, yy),
                    (xx - 5, yy),
                    (xx - 6, yy),
                    (xx - 7, yy)
                ]
            ],
        }
    }
}


impl Iterator for RookMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x;
        let mut y;
        loop {
            if let Some(arr) = self.data.get(self.first_counter) {
                match arr.get(self.second_counter) {
                    Some((xx @ 0..=7, yy @ 0..=7)) => {
                        x = *xx as usize;
                        y = *yy as usize;
                        match self.board.board[x][y].can_move_to_and_through(self.white) {
                            (true, true) => {
                                self.second_counter += 1;
                                return Some(Move::RookMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            (true, false) => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                                return Some(Move::RookMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            _ => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                            }
                        }
                    }
                    _ => {
                        self.first_counter += 1;
                        self.second_counter = 0
                    }
                }
            } else {
                return None;
            }
        }
    }
}


pub struct QueenMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    first_counter: usize,
    second_counter: usize,
    white: bool,
    data: [[(isize, isize); 7]; 8],
}


impl QueenMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> QueenMoveGenerator {
        let xx = x as isize;
        let yy = y as isize;
        QueenMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            first_counter: 0,
            second_counter: 0,
            data: [
                [
                    (xx + 1, yy + 1),
                    (xx + 2, yy + 2),
                    (xx + 3, yy + 3),
                    (xx + 4, yy + 4),
                    (xx + 5, yy + 5),
                    (xx + 6, yy + 6),
                    (xx + 7, yy + 7)
                ],
                [
                    (xx + 1, yy - 1),
                    (xx + 2, yy - 2),
                    (xx + 3, yy - 3),
                    (xx + 4, yy - 4),
                    (xx + 5, yy - 5),
                    (xx + 6, yy - 6),
                    (xx + 7, yy - 7)
                ],
                [
                    (xx - 1, yy + 1),
                    (xx - 2, yy + 2),
                    (xx - 3, yy + 3),
                    (xx - 4, yy + 4),
                    (xx - 5, yy + 5),
                    (xx - 6, yy + 6),
                    (xx - 7, yy + 7)
                ],
                [
                    (xx - 1, yy - 1),
                    (xx - 2, yy - 2),
                    (xx - 3, yy - 3),
                    (xx - 4, yy - 4),
                    (xx - 5, yy - 5),
                    (xx - 6, yy - 6),
                    (xx - 7, yy - 7)
                ],
                [
                    (xx, yy + 1),
                    (xx, yy + 2),
                    (xx, yy + 3),
                    (xx, yy + 4),
                    (xx, yy + 5),
                    (xx, yy + 6),
                    (xx, yy + 7)
                ],
                [
                    (xx, yy - 1),
                    (xx, yy - 2),
                    (xx, yy - 3),
                    (xx, yy - 4),
                    (xx, yy - 5),
                    (xx, yy - 6),
                    (xx, yy - 7)
                ],
                [
                    (xx + 1, yy),
                    (xx + 2, yy),
                    (xx + 3, yy),
                    (xx + 4, yy),
                    (xx + 5, yy),
                    (xx + 6, yy),
                    (xx + 7, yy)
                ],
                [
                    (xx - 1, yy),
                    (xx - 2, yy),
                    (xx - 3, yy),
                    (xx - 4, yy),
                    (xx - 5, yy),
                    (xx - 6, yy),
                    (xx - 7, yy)
                ]
            ],
        }
    }
}

impl Iterator for QueenMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x;
        let mut y;
        loop {
            if let Some(arr) = self.data.get(self.first_counter) {
                match arr.get(self.second_counter) {
                    Some((xx @ 0..=7, yy @ 0..=7)) => {
                        x = *xx as usize;
                        y = *yy as usize;
                        match self.board.board[x][y].can_move_to_and_through(self.white) {
                            (true, true) => {
                                self.second_counter += 1;
                                return Some(Move::QueenMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            (true, false) => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                                return Some(Move::QueenMove(self.x, self.y, x, y, self.board.board[x][y]));
                            }
                            _ => {
                                self.first_counter += 1;
                                self.second_counter = 0;
                            }
                        }
                    }
                    _ => {
                        self.first_counter += 1;
                        self.second_counter = 0
                    }
                }
            } else {
                return None;
            }
        }
    }
}


pub struct KingMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    white: bool,
    counter: usize,
    data: [(isize, isize); 8],
}


impl KingMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> KingMoveGenerator {
        let xx = x as isize;
        let yy = y as isize;
        KingMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            counter: 0,
            data: [
                (xx - 1, yy - 1),
                (xx - 1, yy + 1),
                (xx + 1, yy - 1),
                (xx + 1, yy + 1),
                (xx + 1, yy),
                (xx - 1, yy),
                (xx, yy + 1),
                (xx, yy - 1)
            ],
        }
    }
}


impl Iterator for KingMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return match self.data.get(self.counter) {
                None => match (self.counter, self.white) {
                    (8, true) if self.board.king_moved[0] == 0 && self.board.rook_moved[0] == 0 && self.board.can_castle(7, 5, 6) => {
                        self.counter += 1;
                        Some(Move::Castling(7, 4, 7, 6))
                    }
                    (8, false) if self.board.king_moved[1] == 0 && self.board.rook_moved[2] == 0 && self.board.can_castle(0, 5, 6) => {
                        self.counter += 1;
                        Some(Move::Castling(0, 4, 7, 6))
                    }
                    (9, true) if self.board.king_moved[0] == 0 && self.board.rook_moved[1] == 0 && self.board.can_castle(7, 3, 2) => {
                        self.counter += 1;
                        Some(Move::Castling(7, 4, 7, 2))
                    }
                    (9, false) if self.board.king_moved[1] == 0 && self.board.rook_moved[3] == 0 && self.board.can_castle(0, 3, 2) => {
                        self.counter += 1;
                        Some(Move::Castling(0, 4, 7, 2))
                    }
                    _ => None
                },
                Some((xx @ 0..=7, yy @ 0..=7)) => {
                    let x = *xx as usize;
                    let y = *yy as usize;
                    if self.board.board[x][y].can_move_to(self.white) {
                        self.counter += 1;
                        Some(Move::KingMove(self.x, self.y, x, y, self.board.board[x][y]))
                    } else {
                        self.counter += 1;
                        continue
                    }
                }
                _ => {
                    self.counter += 1;
                    continue;
                }
            };
        }
    }
}


pub struct PawnMoveGenerator<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
    white: bool,
    counter: usize,
    promotion_counter: usize,
    starting_square: bool,
    movement: usize
}


impl PawnMoveGenerator<'_> {
    pub fn new(board: &Board, x: usize, y: usize) -> PawnMoveGenerator {
        let (movement, promotion_counter, starting_square) = match (board.white_to_move, x) {
            (true, 1) => (0, 1, false),
            (false, 6) => (2, 1, false),
            (true, 6) => (0, 0, true),
            (false, 1) => (2, 0, true),
            (true, _) => (0, 0, false),
            (false, _) => (2, 0, false),
        };
        PawnMoveGenerator {
            board,
            x,
            y,
            white: board.white_to_move,
            counter: 0,
            promotion_counter,
            starting_square,
            movement
        }
    }
}


impl Iterator for PawnMoveGenerator<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x= self.x + self.movement - 1;
        let mut y;
        loop {
            match (self.counter, self.promotion_counter) {
                (0, 0) => {
                    self.counter = 1;
                    if self.y == 0 {
                        continue;
                    }
                    y = self.y - 1;
                    if self.board.board[x][y].can_move_to_and_through(self.white) == (true, false) {
                        return Some(Move::PawnCapture(self.x, self.y, x, y, self.board.board[x][y]))
                    } else if self.board.en_passant_array[x][y] == self.board.clock {
                        return Some(Move::EnPassant(self.x, self.y, x, y, self.board.board[self.x][y]))
                    }
                },
                (0, 1) => {
                    if self.y == 0 {
                        continue;
                    }
                    y = self.y - 1;
                    if self.board.board[x][y].can_move_to_and_through(self.white) == (true, false) {
                        self.promotion_counter = 2;
                        return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::Q{white: self.white}))
                    } else {
                        self.counter = 1;
                    }
                },
                (0, 2) => {
                    self.promotion_counter = 3;
                    y = self.y - 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::N{white: self.white}))
                },
                (0, 3) => {
                    self.promotion_counter = 4;
                    y = self.y - 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::B{white: self.white}))
                },
                (0, 4) => {
                    self.counter = 1;
                    self.promotion_counter = 1;
                    y = self.y - 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::R{white: self.white}))
                },
                (1, 0) => {
                    self.counter = 2;
                    if self.y == 7 {
                        continue;
                    }
                    y = self.y + 1;
                    if self.board.board[x][y].can_move_to_and_through(self.white) == (true, false) {
                        return Some(Move::PawnCapture(self.x, self.y, x, y, self.board.board[x][y]))
                    } else if self.board.en_passant_array[x][y] == self.board.clock {
                        return Some(Move::EnPassant(self.x, self.y, x, y, self.board.board[self.x][y]))
                    }
                },
                (1, 1) => {
                    if self.y == 7 {
                        continue;
                    }
                    y = self.y + 1;
                    if self.board.board[x][y].can_move_to_and_through(self.white) == (true, false) {
                        self.promotion_counter = 2;
                        return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::Q{white: self.white}))
                    } else {
                        self.counter = 2;
                    }
                },
                (1, 2) => {
                    self.promotion_counter = 3;
                    y = self.y + 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::N{white: self.white}))
                },
                (1, 3) => {
                    self.promotion_counter = 4;
                    y = self.y + 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::B{white: self.white}))
                },
                (1, 4) => {
                    self.counter = 2;
                    self.promotion_counter = 1;
                    y = self.y + 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, y, self.board.board[x][y], Square::R{white: self.white}))
                },
                (2, 0) => {
                    self.counter = 3;
                    if self.board.board[x][self.y].is_empty() {
                        return Some(Move::PawnPush(self.x, self.y, x, self.y))
                    } else if self.board.en_passant_array[x][self.y] == self.board.clock {
                        return Some(Move::EnPassant(self.x, self.y, x, self.y, self.board.board[self.x][self.y]))
                    }
                },
                (2, 1) => {
                    if self.board.board[x][self.y].is_empty() {
                        self.promotion_counter = 2;
                        return Some(Move::PawnPromotion(self.x, self.y, x, self.y, Square::E, Square::Q{white: self.white}))
                    } else {
                        self.counter = 2;
                    }
                },
                (2, 2) => {
                    self.promotion_counter = 3;
                    return Some(Move::PawnPromotion(self.x, self.y, x, self.y, Square::E, Square::N{white: self.white}))
                },
                (2, 3) => {
                    self.promotion_counter = 4;
                    return Some(Move::PawnPromotion(self.x, self.y, x, self.y, Square::E, Square::B{white: self.white}))
                },
                (2, 4) => {
                    self.counter = 3;
                    self.promotion_counter = 1;
                    return Some(Move::PawnPromotion(self.x, self.y, x, self.y, Square::E, Square::R{white: self.white}))
                },
                _ => {
                    return if self.starting_square {
                        x = self.x + self.movement + self.movement - 2;
                        if self.board.board[self.x + self.movement - 1].is_empty() && self.board.board[x][self.y].is_empty() {
                            self.starting_square = false;
                            Some(Move::TwoSquarePawnMove(self.x, self.y, x, self.y))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        }
    }
}

