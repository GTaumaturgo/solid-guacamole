use strum::IntoEnumIterator;
use strum_macros::EnumIter; // 0.17.1

pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct ChessPiece {
    pub typpe: PieceType,
    pub color: PlayerColor,
}

pub mod bitboard;
pub mod position;
