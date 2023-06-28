use crate::utils::{Square, Board};
use rand::{SeedableRng, RngCore};
use rand::rngs::StdRng;


#[derive(Debug)]
pub struct Hasher {
    piece_map: [[[u64; 8]; 8]; 13],
    // ignore whose turn it is because i store these separately anyways
    // black_to_move: u64,
    castling_rights: [u64; 4],
    en_passant_file: [u64; 9]
}


impl Hasher {
    pub fn new(seed: u64) -> Hasher {
        let mut gen = StdRng::seed_from_u64(seed);
        let mut piece_map: [[[u64; 8]; 8]; 13] = [[[0; 8]; 8]; 13];
        let mut castling_rights: [u64; 4] = [0; 4];
        // 9 will be used instead of none
        let mut en_passant_file: [u64; 9] = [0; 9];
        for i in 1..13 {
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
            // black_to_move: gen.next_u64()
        }
    }
    pub fn get_hash_values(&self) -> Vec<u64> {
        let mut output = Vec::new();
        for i in 1..13 {
            for j in 0..8 {
                for k in 0..8 {
                    output.push(self.piece_map[i][j][k]);
                }
            }
        }
        for i in 0..4 {
            output.push(self.castling_rights[i]);
        }
        for i in 0..4 {
            output.push(self.en_passant_file[i]);
        }
        output
    }
    pub fn get_pawn_zobrist(&self, board: &Board) -> u64 {
        let mut output = 0;
        for i in 0..8 {
            for j in 0..8 {
                match board.board[i][j] {
                    Square::PW => { output ^= self.piece_map[0][i][j]; },
                    Square::PB => { output ^= self.piece_map[1][i][j]; },
                    _ => ()
                }
            }
        }
        output
    }
    pub fn get_zobrist_hash(&self, board: &Board) -> u64 {
        let mut output = 0;
        for i in 0..8 {
            for j in 0..8 {
                output ^= self.piece_map[board.board[i][j] as usize][i][j];
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
        output ^= self.en_passant_file[board.en_passant_hash_index[board.clock]];
        // if !board.white_to_move {
        //     output ^= self.black_to_move;
        // }
        // if board.white_to_move {
            // for i in 0..8 {
            //     if board.en_passant_array[2][i] == board.clock {
            //         return output ^ self.en_passant_file[i]
            //     }
            // }
        // } else {
        //     output ^= self.black_to_move;
            // for i in 0..8 {
            //     if board.en_passant_array[5][i] == board.clock {
            //         return output ^ self.en_passant_file[i]
            //     }
            // }
        // }
        output
    }
}