pub const NUM_SQUARES: usize = 64;
pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

pub type BitB64 = u64;

pub const empty_board : BitB64 = 0u64;
pub const full_board : BitB64 = u64::MAX;
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
pub struct Bitboard {
    pub pawns: BitB64,
    pub knights: BitB64,
    pub bishops: BitB64,
    pub rooks: BitB64,
    pub queens: BitB64,
    pub king: BitB64,
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

    pub fn get_pieces(&self) -> BitB64 {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }
}

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
