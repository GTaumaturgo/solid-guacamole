use crate::chess::bitboard::BitB64;
use crate::chess::position::Position;
use crate::chess::PieceType;
use crate::chess::PlayerColor;

use super::PositionEvaluator;
use crate::chess::bitboard::{BitArraySize, PlayerBitboard, SpecialMoveType};
use crate::strum::IntoEnumIterator;
use std::collections::HashMap;

pub struct PieceCoordinateEvaluator {
    piece_values: HashMap<(PieceType, PlayerColor), HashMap<u8, i32>>,
}

fn get_bishop_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match (rank, file) {
            (0, 2) | (0, 5) | (1, 3) | (1, 4) | (2, 2) | (2, 5) | (3, 3) | (3, 4) => 20, // Good development squares
            (2, 1)
            | (2, 6)
            | (3, 2)
            | (3, 5)
            | (4, 3)
            | (4, 4)
            | (5, 2)
            | (5, 5)
            | (6, 3)
            | (6, 4) => 30, // Central squares
            (4, 1) | (4, 6) | (5, 0) | (5, 7) | (6, 1) | (6, 6) => 20,                   // Outposts
            (1, 1) | (1, 6) | (6, 0) | (6, 7) => 0, // Slightly less desirable
            (0, 0) | (0, 7) | (7, 0) | (7, 7) => -20, // Corners are bad for bishops
            _ => 10,                                // Other squares
        };
        result.insert(i as u8, value);
    }
    result
}

fn get_knight_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match (rank, file) {
            (2, 3) | (2, 4) | (3, 2) | (3, 5) | (4, 3) | (4, 4) | (5, 2) | (5, 5) => 30, // Centermost squares
            (1, 2)
            | (1, 5)
            | (2, 2)
            | (2, 5)
            | (3, 3)
            | (3, 4)
            | (4, 2)
            | (4, 5)
            | (5, 3)
            | (5, 4)
            | (6, 2)
            | (6, 5) => 20, // Slightly outward
            (0, 1)
            | (0, 6)
            | (1, 3)
            | (1, 4)
            | (2, 1)
            | (2, 6)
            | (3, 1)
            | (3, 6)
            | (4, 1)
            | (4, 6)
            | (5, 1)
            | (5, 6)
            | (6, 3)
            | (6, 4)
            | (7, 1)
            | (7, 6) => 10, // Further outward
            (0, 0)
            | (0, 7)
            | (1, 0)
            | (1, 7)
            | (2, 0)
            | (2, 7)
            | (3, 0)
            | (3, 7)
            | (4, 0)
            | (4, 7)
            | (5, 0)
            | (5, 7)
            | (6, 0)
            | (6, 7)
            | (7, 0)
            | (7, 7) => 0, // Corner squares
            _ => -10, // Other squares
        };
        result.insert(i as u8, value);
    }
    result
}

fn get_pawn_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match rank {
            0 => 0,              // Impossible
            1 => 0,              // Starting position
            2 => 10,             // Second rank
            3 => 20,             // Third rank
            4 => 30,             // Fourth rank
            5 => 50,             // Fifth rank
            6 => 80,             // Sixth rank (close to promotion)
            7 => 0,              // Promoted (not relevant for PST)
            _ => unreachable!(), // Should not happen
        } + match file {
            3 | 4 => 10, // Bonus for center files
            2 | 5 => 5,  // Small bonus for near-center files
            _ => 0,      // No bonus for edge files
        };
        result.insert(i as u8, value);
    }
    result
}

fn get_queen_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match (rank, file) {
            (3, 3) | (3, 4) | (4, 3) | (4, 4) => 30, // Centermost squares
            (2..=5, 2..=5) => 20,                    // Central squares
            _ => 0,                                  // Otherwise neutral
        };
        result.insert(i as u8, value);
    }
    result
}

fn get_rook_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match (rank, file) {
            (0, 0) | (0, 7) | (7, 0) | (7, 7) => 0, // Slightly less valuable on corners
            (0..=7, 0) | (0..=7, 7) => 5,           // Good on open files
            _ => -5,                                // Generally prefers open files
        };
        result.insert(i as u8, value);
    }
    result
}

fn get_king_map() -> HashMap<u8, i32> {
    let mut result = HashMap::new();
    for i in 0..64 {
        let rank = i / 8;
        let file = i % 8;
        let value = match (rank, file) {
            // Favor castled positions (adjust as needed)
            (0, 6) | (0, 2) => 20, // Kingside and queenside castling
            (1, 5) | (1, 3) => 10, // Squares near castling positions

            // Discourage exposed positions
            (2..=5, 2..=5) => -10, // Central squares
            _ => 0,                // Neutral for other squares
        };

        result.insert(i as u8, value);
    }
    result
}

fn reverse_map(original_map: &HashMap<u8, i32>) -> HashMap<u8, i32> {
    let mut reversed_map = HashMap::new();
    for (key, value) in original_map {
        let reversed_key = 63 - key; // Calculate the mirrored square
        reversed_map.insert(reversed_key, *value);
    }
    reversed_map
}

impl PieceCoordinateEvaluator {
    pub fn new() -> Self {
        let mut piece_values = HashMap::new();

        let mut func_by_type: HashMap<PieceType, fn() -> HashMap<u8, i32>> = HashMap::new();
        func_by_type.insert(PieceType::Pawn, get_pawn_map);
        func_by_type.insert(PieceType::Knight, get_knight_map);
        func_by_type.insert(PieceType::Bishop, get_bishop_map);
        func_by_type.insert(PieceType::Rook, get_rook_map);
        func_by_type.insert(PieceType::Queen, get_queen_map);
        func_by_type.insert(PieceType::King, get_king_map);

        for piece_type in PieceType::iter() {
            let map = func_by_type[&piece_type]();
            piece_values.insert((piece_type, PlayerColor::Black), reverse_map(&map));
            piece_values.insert((piece_type, PlayerColor::White), map);
        }

        PieceCoordinateEvaluator {
            piece_values: piece_values,
        }
    }

    fn count_pieces_of_type(
        &self,
        mut piece_set: BitB64,
        piece_type: PieceType,
        player_color: PlayerColor,
    ) -> i32 {
        let mut result = 0;
        let piece_map = &self.piece_values[&(piece_type, player_color)];
        while piece_set != 0 {
            let piece_id = piece_set.trailing_zeros() as u8;
            piece_set ^= u64::nth(piece_id);
            result += piece_map[&piece_id];
        }
        result
    }
}

impl PositionEvaluator for PieceCoordinateEvaluator {
    fn evaluate(&self, position: &Position) -> i32 {
        let mut score = 0;

        let (white, black) = (position.white, position.black);
        for piece_type in PieceType::iter() {
            score += self.count_pieces_of_type(
                *white.pieces(piece_type),
                piece_type,
                PlayerColor::White,
            );
            score -= self.count_pieces_of_type(
                *black.pieces(piece_type),
                piece_type,
                PlayerColor::Black,
            );
        }
        score
    }
}
