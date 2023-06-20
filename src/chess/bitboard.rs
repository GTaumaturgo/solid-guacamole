use super::{PieceType, PlayerColor};

pub const NUM_SQUARES: usize = 64;
pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

pub type BitB64 = u64;

pub const empty_board: BitB64 = 0u64;
pub const full_board: BitB64 = u64::MAX;
// Usage: bitb!(3) -> 1u64 << (3 * 8 + 4)
#[macro_export]
macro_rules! bitb {
    ($x:expr) => {
        1u64 << ($x as u8)
    };
}
#[macro_export]
macro_rules! bitb32 {
    ($x:expr) => {
        1u32 << ($x as u8)
    };
}
#[macro_export]
macro_rules! bitb16 {
    ($x:expr) => {
        1u16 << ($x as u8)
    };
}
#[macro_export]
macro_rules! bitb8 {
    ($x:expr) => {
        1u8 << ($x as u8)
    };
}

// Bitboard with information about the pieces of one PlayerColor.
#[derive(Clone, Copy)]
pub struct Bitboard {
    pub pawns: BitB64,
    pub knights: BitB64,
    pub bishops: BitB64,
    pub rooks: BitB64,
    pub queens: BitB64,
    pub king: BitB64,
}

impl Bitboard {
    pub fn new(color: PlayerColor) -> Bitboard {
        match color {
            PlayerColor::White => Bitboard {
                pawns: bitb!(8)
                    | bitb!(9)
                    | bitb!(10)
                    | bitb!(11)
                    | bitb!(12)
                    | bitb!(13)
                    | bitb!(14)
                    | bitb!(15),
                knights: bitb!(1) | bitb!(6),
                bishops: bitb!(2) | bitb!(5),
                rooks: bitb!(0) | bitb!(7),
                queens: bitb!(3),
                king: bitb!(4), 
            },
            PlayerColor::Black => Bitboard {
                pawns: bitb!(63 - 8)
                    | bitb!(63 - 9)
                    | bitb!(63 - 10)
                    | bitb!(63 - 11)
                    | bitb!(63 - 12)
                    | bitb!(63 - 13)
                    | bitb!(63 - 14)
                    | bitb!(63 - 15),
                knights: bitb!(63 - 1) | bitb!(63 - 6),
                bishops: bitb!(63 - 2) | bitb!(63 - 5),
                rooks: bitb!(63 - 0) | bitb!(63 - 7),
                queens: bitb!(63 - 4), // Swapped queen and king
                king: bitb!(63 - 3), 
            },
        }
    }

    pub fn all_pieces(&self) -> BitB64 {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }

    pub fn pieces(&self, typpe: PieceType) -> &BitB64 {
        match typpe {
            PieceType::Pawn => &self.pawns,
            PieceType::Knight => &self.knights,
            PieceType::Bishop => &self.bishops,
            PieceType::Rook => &self.rooks,
            PieceType::Queen => &self.queens,
            PieceType::King => &self.king,
        }
    }

    pub fn mut_pieces(&mut self, typpe: PieceType) -> &mut BitB64 {
        match typpe {
            PieceType::Pawn => &mut self.pawns,
            PieceType::Knight => &mut self.knights,
            PieceType::Bishop => &mut self.bishops,
            PieceType::Rook => &mut self.rooks,
            PieceType::Queen => &mut self.queens,
            PieceType::King => &mut self.king,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BitboardMove {
    pub from: u8,
    pub to: u8,
}

pub struct MoveScore {
    pub score: i64,
    // pub confidence: u16,
    // pub depth: u8,
}

pub struct ScoredMove {
    pub score: MoveScore,
    pub mov: BitboardMove,
}
