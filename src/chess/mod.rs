use strum::IntoEnumIterator;
use strum_macros::EnumIter; // 0.17.1

#[derive(Clone, Copy)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn other(color: PlayerColor) -> PlayerColor {
        match color {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Copy, Clone, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct ChessPiece {
    pub typpe: PieceType,
    pub color: PlayerColor,
}

pub mod bitboard;
pub mod position;
