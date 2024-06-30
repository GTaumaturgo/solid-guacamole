use crate::chess::bitboard::BitB64;
use crate::chess::position::Position;
use crate::chess::PieceType;
use crate::chess::PlayerColor;

use crate::strum::IntoEnumIterator;

use super::PositionEvaluator;
use crate::chess::bitboard::{BitArraySize, PlayerBitboard, SpecialMoveType};
use std::collections::HashMap;

pub struct MaterialEvaluator {
    piece_values: HashMap<PieceType, i32>,
}

impl MaterialEvaluator {
    pub fn new() -> Self {
        let mut piece_values = HashMap::new();
        piece_values.insert(PieceType::Pawn, 105);
        piece_values.insert(PieceType::Knight, 310);
        piece_values.insert(PieceType::Bishop, 325);
        piece_values.insert(PieceType::Rook, 500);
        piece_values.insert(PieceType::Queen, 900);
        piece_values.insert(PieceType::King, 0);

        MaterialEvaluator {
            piece_values: piece_values,
        }
    }

    fn count_pieces_of_type(&self, mut piece_set: BitB64, piece_type: PieceType) -> i32 {
        let mut result = 0;
        while piece_set != 0 {
            let piece_id = piece_set.trailing_zeros() as u8;
            piece_set ^= u64::nth(piece_id);
            result += self.piece_values[&piece_type];
        }
        result
    }
}

impl PositionEvaluator for MaterialEvaluator {
    fn evaluate(&self, position: &Position) -> i32 {
        let mut score = 0;
        let (white, black) = (position.white, position.black);
        for piece_type in PieceType::iter() {
            score += self.count_pieces_of_type(*white.pieces(piece_type), piece_type);
            score -= self.count_pieces_of_type(*black.pieces(piece_type), piece_type);
        }
        return score;
    }
}
