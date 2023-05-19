pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

pub struct ChessPiece {
    piece: PieceType,
    color: Color,
}

const NUM_SQUARES: usize = 64;
