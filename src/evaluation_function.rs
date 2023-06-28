use std::cmp::max;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::utils::{Board, Square, Evaluation};
use crate::eval::{EYEING_VALUES, KING_ATTACK_VALUES, KING_FEDENCE_NOZE, KING_SAFETY_VALUES, SQUARE_IMPORTANCE, MATERIAL_IMPORTANCE, PAWN_BLOCKING_PENALTIES, DISTANCE_TO_CORNER, KING_DISTANCE, KING_DANGER_VALUES, PAWN_PENALTIES};
use crate::hashing::Hasher;


// just nerfed everything except material (saw some bullshit)
pub const MATERIAL_WEIGHT: i64 = 2;
pub const POSITIONING_WEIGHT: i64 = 100;
pub const SAFETY_WEIGHT: i64 = 100;
pub const ACTIVITY_WEIGHT: i64 = 10;
pub const EYE_WEIGHT: i64 = 20;
pub const PAWN_STRUCTURE_WEIGHT: i64 = 125;
pub const PAWN_BLOCKING_WEIGHT: i64 = 100;
pub const MOPUP_WEIGHT: i64 = 100;
pub const MOPUP_ATTACKER_WEIGHT: i64 = 3;


impl Board {
    pub fn compute_evaluation(&self, hasher: &Hasher, pawn_structure_cache: &mut HashMap<u64, i64>) -> i64 {
        let mut piece;
        let mut output;
        let mut eyed_values = 0;
        let mut blocking_value = 0;
        let mut white_has_pieces = false;
        let mut black_has_pieces = false;
        let mut piece_distance;
        let mut weighted_activity = 0;
        // white pieces attacking black king
        let mut white_attackers = 0;
        // black pieces attacking white king
        let mut black_attackers = 0;
        output = MATERIAL_IMPORTANCE[self.evaluation.total_material as usize] * MATERIAL_WEIGHT * self.evaluation.material;
        // output = MATERIAL_WEIGHT * self.evaluation.material;
        for x in 0..8 {
            for y in 0..8 {
                weighted_activity += SQUARE_IMPORTANCE[x][y] *
                    (self.visibility.map[x][y].total[0].len() as i64 - self.visibility.map[x][y].total[1].len() as i64);
                match self.board[x][y] {
                    Square::E => continue,
                    Square::PW => blocking_value -= PAWN_BLOCKING_PENALTIES[0][self.board[x - 1][y] as usize][x][y],
                    Square::PB => blocking_value -= PAWN_BLOCKING_PENALTIES[1][self.board[x + 1][y] as usize][x][y],
                    Square::NW | Square::BW | Square::RW | Square::QW => white_has_pieces = true,
                    Square::NB | Square::BB | Square::RB | Square::QB => black_has_pieces = true,
                    _ => ()
                }
                piece = self.board[x][y] as usize;
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
        match (white_has_pieces, black_has_pieces) {
            (true, true) => {},
            (true, false) => {
                // pries tai sitas buvo padaugintas ir is material balance
                // bet tada mano AI saccino queen kad pralaiminetu maziau
                output -= MOPUP_WEIGHT * DISTANCE_TO_CORNER[self.kings[1][0]][self.kings[1][1]];
                piece_distance = 0;
                for x in 0..8 {
                    for y in 0..8 {
                        piece_distance += KING_DANGER_VALUES[self.board[x][y] as usize] * KING_DISTANCE[self.kings[1][0]][self.kings[1][1]][x][y];
                    }
                }
                output -= MOPUP_ATTACKER_WEIGHT * piece_distance;
                return output;
            },
            (false, true) => {
                output += MOPUP_WEIGHT * DISTANCE_TO_CORNER[self.kings[0][0]][self.kings[0][1]];
                piece_distance = 0;
                for x in 0..8 {
                    for y in 0..8 {
                        piece_distance += KING_DANGER_VALUES[self.board[x][y] as usize] * KING_DISTANCE[self.kings[1][0]][self.kings[1][1]][x][y];
                    }
                }
                output += MOPUP_ATTACKER_WEIGHT * piece_distance;
                return output;
            },
            (false, false) => {
                output += POSITIONING_WEIGHT * self.evaluation.positioning;
                output += PAWN_BLOCKING_WEIGHT * blocking_value;
                return output + eyed_values * EYE_WEIGHT;
            }
        }
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
        output += POSITIONING_WEIGHT * self.evaluation.positioning;
        output += PAWN_BLOCKING_WEIGHT * blocking_value;
        output += SAFETY_WEIGHT * (KING_SAFETY_VALUES[white_attackers] - KING_SAFETY_VALUES[black_attackers]);
        output += eyed_values * EYE_WEIGHT;
        output + weighted_activity * ACTIVITY_WEIGHT
    }
    pub unsafe fn sp_compute_evaluation(&self, hasher: &Hasher, pawn_structure_cache: *mut Mutex<HashMap<u64, i64>>) -> i64 {
        debug_assert!(self.evaluation.total_material >= 0);
        let mut piece;
        let mut output;
        let mut eyed_values = 0;
        let mut blocking_value = 0;
        let mut white_has_pieces = false;
        let mut black_has_pieces = false;
        let mut piece_distance;
        let mut weighted_activity = 0;
        // white pieces attacking black king
        let mut white_attackers = 0;
        // black pieces attacking white king
        let mut black_attackers = 0;
        output = MATERIAL_IMPORTANCE[self.evaluation.total_material as usize] * MATERIAL_WEIGHT * self.evaluation.material;
        // output = MATERIAL_WEIGHT * self.evaluation.material;
        for x in 0..8 {
            for y in 0..8 {
                weighted_activity += SQUARE_IMPORTANCE[x][y] *
                    (self.visibility.map[x][y].total[0].len() as i64 - self.visibility.map[x][y].total[1].len() as i64);
                match self.board[x][y] {
                    Square::E => continue,
                    Square::PW => blocking_value -= PAWN_BLOCKING_PENALTIES[0][self.board[x - 1][y] as usize][x][y],
                    Square::PB => blocking_value -= PAWN_BLOCKING_PENALTIES[1][self.board[x + 1][y] as usize][x][y],
                    Square::NW | Square::BW | Square::RW | Square::QW => white_has_pieces = true,
                    Square::NB | Square::BB | Square::RB | Square::QB => black_has_pieces = true,
                    _ => ()
                }
                piece = self.board[x][y] as usize;
                for (px, py) in self.visibility.map[x][y].total[0].iter() {
                    eyed_values += EYEING_VALUES[self.board[*px][*py] as usize][piece];
                }
                for (px, py) in self.visibility.map[x][y].total[1].iter() {
                    eyed_values += EYEING_VALUES[self.board[*px][*py] as usize][piece];
                }
            }
        }
        let pawn_hash = hasher.get_pawn_zobrist(self);
        if let Ok(mut guard) = (*pawn_structure_cache).lock() {
            if let Some(eval) = guard.get(&pawn_hash) {
                output += eval;
            } else {
                let eval = PAWN_STRUCTURE_WEIGHT * self.eval_pawns();
                guard.insert(pawn_hash, eval);
                output += eval;
            }
        }
        match (white_has_pieces, black_has_pieces) {
            (true, true) => {},
            (true, false) => {
                // pries tai sitas buvo padaugintas ir is material balance
                // bet tada mano AI saccino queen kad pralaiminetu maziau
                output -= MOPUP_WEIGHT * DISTANCE_TO_CORNER[self.kings[1][0]][self.kings[1][1]];
                piece_distance = 0;
                for x in 0..8 {
                    for y in 0..8 {
                        piece_distance += KING_DANGER_VALUES[self.board[x][y] as usize] * KING_DISTANCE[self.kings[1][0]][self.kings[1][1]][x][y];
                    }
                }
                output -= MOPUP_ATTACKER_WEIGHT * piece_distance;
                return output;
            },
            (false, true) => {
                output += MOPUP_WEIGHT * DISTANCE_TO_CORNER[self.kings[0][0]][self.kings[0][1]];
                piece_distance = 0;
                for x in 0..8 {
                    for y in 0..8 {
                        piece_distance += KING_DANGER_VALUES[self.board[x][y] as usize] * KING_DISTANCE[self.kings[1][0]][self.kings[1][1]][x][y];
                    }
                }
                output += MOPUP_ATTACKER_WEIGHT * piece_distance;
                return output;
            },
            (false, false) => {
                output += POSITIONING_WEIGHT * self.evaluation.positioning;
                output += PAWN_BLOCKING_WEIGHT * blocking_value;
                return output + eyed_values * EYE_WEIGHT;
            }
        }
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
        output += POSITIONING_WEIGHT * self.evaluation.positioning;
        output += PAWN_BLOCKING_WEIGHT * blocking_value;
        output += SAFETY_WEIGHT * (KING_SAFETY_VALUES[white_attackers] - KING_SAFETY_VALUES[black_attackers]);
        output += eyed_values * EYE_WEIGHT;
        output + weighted_activity * ACTIVITY_WEIGHT
    }
    pub fn eval_pawns(&self) -> i64 {
        // this is dumb as hell i will not be touching this code
        let mut recycling;
        let mut output= 0;
        let mut white_back_coords = [0; 8];
        let mut white_front_coords = [7; 8];
        let mut black_back_coords = [7; 8];
        let mut black_front_coords = [0; 8];
        let mut white_counts = [0; 8];
        let mut black_counts = [0; 8];
        for y in 0..8 {
            for x in 0..8 {
                match self.board[x][y] {
                    Square::PW => {
                        white_counts[y] += 1;
                        white_back_coords[y] = x;
                    },
                    Square::PB => {
                        black_front_coords[y] = x;
                    }
                    _ => ()
                }
                // recycle memory
                recycling = 7 - x;
                match self.board[recycling][y] {
                    Square::PB => {
                        black_counts[y] += 1;
                        black_back_coords[y] = recycling;
                    },
                    Square::PW => {
                        white_front_coords[y] = recycling;
                    },
                    _ => ()
                }
            }
        }
        if white_counts[0] > 0 {
            if white_counts[1] == 0 {
                output -= PAWN_PENALTIES[0];
            } else if white_back_coords[0] > white_back_coords[1] {
                output -= PAWN_PENALTIES[1];
            } else if white_front_coords[1] > 2 + white_front_coords[0] {
                output -= PAWN_PENALTIES[4]
            }
            output -= PAWN_PENALTIES[2] * (white_counts[0] - 1);
        } else if black_counts[0] > 0 && black_front_coords[0] >= white_back_coords[1] {
            output -= PAWN_PENALTIES[3];
        }
        if black_counts[0] > 0 {
            if black_counts[1] == 0 {
                output += PAWN_PENALTIES[0];
            } else if black_back_coords[0] < black_back_coords[1] {
                output += PAWN_PENALTIES[1];
            } else if black_front_coords[0] > 2 + black_front_coords[1] {
                output += PAWN_PENALTIES[4];
            }
            output += PAWN_PENALTIES[2] * (black_counts[0] - 1);
        } else if white_counts[0] > 0 && white_front_coords[0] <= black_back_coords[1] {
            output += PAWN_PENALTIES[3];
        }
        if white_counts[7] > 0 {
            if white_counts[6] == 0 {
                output -= PAWN_PENALTIES[0];
            } else if white_back_coords[7] > white_back_coords[6] {
                output -= PAWN_PENALTIES[1];
            } else if white_front_coords[6] > 2 + white_front_coords[7] {
                output -= PAWN_PENALTIES[4]
            }
            output -= PAWN_PENALTIES[2] * (white_counts[7] - 1);
        } else if black_counts[7] > 0 && black_front_coords[7] >= white_back_coords[6] {
            output -= PAWN_PENALTIES[3];
        }
        if black_counts[7] > 0 {
            if black_counts[6] == 0 {
                output += PAWN_PENALTIES[0];
            } else if black_back_coords[7] < black_back_coords[6] {
                output += PAWN_PENALTIES[1];
            }
            output += PAWN_PENALTIES[2] * (black_counts[7] - 1);
        } else if white_counts[7] > 0 && white_front_coords[7] <= black_back_coords[6] {
            output += PAWN_PENALTIES[3];
        } else if black_front_coords[7] > 2 + black_front_coords[6] {
            output += PAWN_PENALTIES[4];
        }
        for i in 1..7 {
            if white_counts[i] > 0 {
                if white_counts[i - 1] == 0 && white_counts[i + 1] == 0 {
                    // isolated pawn
                    output -= PAWN_PENALTIES[0];
                } else if white_back_coords[i] > white_back_coords[i - 1] && white_back_coords[i] > white_back_coords[i + 1] {
                    // backwards pawn
                    output -= PAWN_PENALTIES[1];
                } else if white_front_coords[i - 1] > 2 + white_front_coords[i] && white_front_coords[i + 1] > 2 + white_front_coords[i] {
                    // outcast pawn
                    output -= PAWN_PENALTIES[4]
                }
                // doubled, tripled, ...
                output -= PAWN_PENALTIES[2] * (white_counts[i] - 1);
            } else if black_counts[i] > 0 && black_front_coords[i] >= white_back_coords[i - 1] && black_front_coords[i] >= white_back_coords[i + 1]{
                // passed pawn
                output -= PAWN_PENALTIES[3];
            }
            if black_counts[i] > 0 {
                if black_counts[i - 1] == 0 && black_counts[i + 1] == 0 {
                    output += PAWN_PENALTIES[0];
                } else if black_back_coords[i] < black_back_coords[i - 1] && black_back_coords[i] < black_back_coords[i + 1] {
                    output += PAWN_PENALTIES[1];
                } else if black_front_coords[i] > 2 + black_front_coords[i - 1] && black_front_coords[i] > 2 + black_front_coords[i + 1] {
                    output += PAWN_PENALTIES[4];
                }
                output += PAWN_PENALTIES[2] * (black_counts[i] - 1);
            } else if white_counts[i] > 0 && white_front_coords[i] <= black_back_coords[i - 1] && white_front_coords[i] <= black_back_coords[i + 1] {
                output += PAWN_PENALTIES[3];
            }
        }
        output
    }
    pub fn evaluate(&self, hasher: &Hasher, pawn_structure_cache: &mut HashMap<u64, i64>) -> i64 {
        if self.white_to_move {
            self.compute_evaluation(hasher, pawn_structure_cache)
        } else {
            -self.compute_evaluation(hasher, pawn_structure_cache)
        }
    }
    pub unsafe fn sp_evaluate(&self, hasher: &Hasher, pawn_structure_cache: *mut Mutex<HashMap<u64, i64>>) -> i64 {
        if self.white_to_move {
            self.sp_compute_evaluation(hasher, pawn_structure_cache)
        } else {
            -self.sp_compute_evaluation(hasher, pawn_structure_cache)
        }
    }
}

