use crate::utils::{Square, Board};
use rand::{SeedableRng, RngCore};
use rand::rngs::StdRng;


#[derive(Debug)]
pub struct Hasher {
    piece_map: [[[u64; 8]; 8]; 12],
    black_to_move: u64,
    castling_rights: [u64; 4],
    en_passant_file: [u64; 8]
}


impl Hasher {
    pub fn new(seed: u64) -> Hasher {
        let mut gen = StdRng::seed_from_u64(seed);
        let mut piece_map: [[[u64; 8]; 8]; 12] = [[[0; 8]; 8]; 12];
        let mut castling_rights: [u64; 4] = [0; 4];
        let mut en_passant_file: [u64; 8] = [0; 8];
        for i in 0..12 {
            for j in 0..8 {
                for k in 0..8 {
                    piece_map[i][j][k] = gen.next_u64();
                }
            }
        }
        for i in 0..4 {
            castling_rights[i] = gen.next_u64();
        }
        for i in 0..8 {
            en_passant_file[i] = gen.next_u64();
        }
        Hasher {
            piece_map,
            castling_rights,
            en_passant_file,
            black_to_move: gen.next_u64()
        }
    }

    pub fn get_zobrist_hash(&self, board: &Board) -> u64 {
        let mut k;
        let mut output = 0;
        for i in 0..8 {
            for j in 0..8 {
                k = match board.board[i][j] {
                    Square::E => continue,
                    Square::P { white: true } => 0,
                    Square::P { white: false } => 1,
                    Square::N { white: true } => 2,
                    Square::N { white: false } => 3,
                    Square::B { white: true } => 4,
                    Square::B { white: false } => 5,
                    Square::R { white: true } => 6,
                    Square::R { white: false } => 7,
                    Square::Q { white: true } => 8,
                    Square::Q { white: false } => 9,
                    Square::K { white: true } => 10,
                Square::K { white: false } => 11
                };
                output ^= self.piece_map[k][i][j];
            }
        }
        if board.king_moved[0] == 0 {
            if board.rook_moved[0] == 0 {
                output ^= self.castling_rights[0];
            }
            if board.rook_moved[1] == 0 {
                output ^= self.castling_rights[1];
            }
        }
        if board.king_moved[1] == 0 {
            if board.rook_moved[2] == 0 {
                output ^= self.castling_rights[2];
            }
            if board.rook_moved[3] == 0 {
                output ^= self.castling_rights[3];
            }
        }
        if board.white_to_move {
            for i in 0..8 {
                if board.en_passant_array[2][i] == board.clock {
                    return output ^ self.en_passant_file[i]
                }
            }
        } else {
            output ^= self.black_to_move;
            for i in 0..8 {
                if board.en_passant_array[5][i] == board.clock {
                    return output ^ self.en_passant_file[i]
                }
            }
        }
        output
    }
}