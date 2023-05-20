mod chess;

// mod move_gen;

mod chess {

const NUM_SQUARES: usize = 64;
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = 8;

pub enum PlayerColor {
    White,
    Black,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Bitboard with information about the pieces of one PlayerColor.
pub struct Bitboard {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    king: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            king: 0,
        }
    }

    pub fn get_pieces(&self) -> u64 {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }
}

// Macro. Usage: bitb!(3) -> 1u64 << (3 * 8 + 4)
macro_rules! bitb {
    ($x:expr) => {
        1u64 << $x
    }
}

pub struct BitboardMove {
    from: u64,
    to: u64,
}


pub struct MoveScore {
    pub score: i64,
    // pub confidence: u16,
    // pub depth: u8,
}

pub struct ScoredMove {
    pub score: Score,
    pub mov: Move,
}



#[derive(PartialEq)]

pub struct ChessPiece {
    piece: PieceType,
    color: PlayerColor,
}


}