use crate::move_gen::knight;

use super::{PieceType, PlayerColor};

pub const NUM_SQUARES: usize = 64;
pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

pub type BitB64 = u64;
pub const EMPTY_BOARD: BitB64 = 0u64;
pub const FULL_BOARD: BitB64 = u64::MAX;

const A1: u8 = 0u8;
const B1: u8 = 1u8;
const C1: u8 = 2u8;
const D1: u8 = 3u8;
const E1: u8 = 4u8;
const F1: u8 = 5u8;
const G1: u8 = 6u8;
const H1: u8 = 7u8;
const A2: u8 = 8u8;
const B2: u8 = 9u8;
const C2: u8 = 10u8;
const D2: u8 = 11u8;
const E2: u8 = 12u8;
const F2: u8 = 13u8;
const G2: u8 = 14u8;
const H2: u8 = 15u8;
const A3: u8 = 16u8;
const B3: u8 = 17u8;
const C3: u8 = 18u8;
const D3: u8 = 19u8;
const E3: u8 = 20u8;
const F3: u8 = 21u8;
const G3: u8 = 22u8;
const H3: u8 = 23u8;
const A4: u8 = 24u8;
const B4: u8 = 25u8;
const C4: u8 = 26u8;
const D4: u8 = 27u8;
const E4: u8 = 28u8;
const F4: u8 = 29u8;
const G4: u8 = 30u8;
const H4: u8 = 31u8;
const A5: u8 = 32u8;
const B5: u8 = 33u8;
const C5: u8 = 34u8;
const D5: u8 = 35u8;
const E5: u8 = 36u8;
const F5: u8 = 37u8;
const G5: u8 = 38u8;
const H5: u8 = 39u8;
const A6: u8 = 40u8;
const B6: u8 = 41u8;
const C6: u8 = 42u8;
const D6: u8 = 43u8;
const E6: u8 = 44u8;
const F6: u8 = 45u8;
const G6: u8 = 46u8;
const H6: u8 = 47u8;
const A7: u8 = 48u8;
const B7: u8 = 49u8;
const C7: u8 = 50u8;
const D7: u8 = 51u8;
const E7: u8 = 52u8;
const F7: u8 = 53u8;
const G7: u8 = 54u8;
const H7: u8 = 55u8;
const A8: u8 = 56u8;
const B8: u8 = 57u8;
const C8: u8 = 58u8;
const D8: u8 = 59u8;
const E8: u8 = 60u8;
const F8: u8 = 61u8;
const G8: u8 = 62u8;
const H8: u8 = 63u8;

pub trait BitArraySize {
    // The actual type.
    type Size;

    // Nth bit turned on: n=2 => 000000010
    fn nth(n: u8) -> Self::Size;
    // Complement of X =>  1111111111 ^ X
    fn compl(x: Self::Size) -> Self::Size;
    //
    fn enable_nth(x: Self::Size, n: u8) -> Self::Size;
    fn disable_nth(x: Self::Size, n: u8) -> Self::Size;
    fn flip_nth(x: Self::Size, n: u8) -> Self::Size;
}

impl BitArraySize for u8 {
    type Size = u8;
    fn nth(n: u8) -> u8 {
        1u8 << n
    }
    fn compl(x: u8) -> u8 {
        u8::MAX ^ x
    }
    fn enable_nth(x: u8, n: u8) -> u8 {
        x | Self::nth(n)
    }
    fn disable_nth(x: u8, n: u8) -> u8 {
        x & Self::compl(Self::nth(n))
    }
    fn flip_nth(x: u8, n: u8) -> u8 {
        x ^ Self::nth(n)
    }
}

impl BitArraySize for u16 {
    type Size = u16;
    fn nth(n: u8) -> u16 {
        1u16 << n
    }
    fn compl(x: u16) -> u16 {
        u16::MAX ^ x
    }
    fn enable_nth(x: u16, n: u8) -> u16 {
        x | Self::nth(n)
    }
    fn disable_nth(x: u16, n: u8) -> u16 {
        x & Self::compl(Self::nth(n))
    }
    fn flip_nth(x: u16, n: u8) -> u16 {
        x ^ Self::nth(n)
    }
}

impl BitArraySize for u32 {
    type Size = u32;
    fn nth(n: u8) -> u32 {
        1u32 << n
    }
    fn compl(x: u32) -> u32 {
        u32::MAX ^ x
    }
    fn enable_nth(x: u32, n: u8) -> u32 {
        x | Self::nth(n)
    }
    fn disable_nth(x: u32, n: u8) -> u32 {
        x & Self::compl(Self::nth(n))
    }
    fn flip_nth(x: u32, n: u8) -> u32 {
        x ^ Self::nth(n)
    }
}

impl BitArraySize for u64 {
    type Size = u64;
    fn nth(n: u8) -> u64 {
        1u64 << n
    }
    fn compl(x: u64) -> u64 {
        u64::MAX ^ x
    }
    fn enable_nth(x: u64, n: u8) -> u64 {
        x | Self::nth(n)
    }
    fn disable_nth(x: u64, n: u8) -> u64 {
        x & Self::compl(Self::nth(n))
    }
    fn flip_nth(x: u64, n: u8) -> u64 {
        x ^ Self::nth(n)
    }
}

// Bitboard with information about the pieces of one PlayerColor.
#[derive(Clone, Copy)]
pub struct PlayerBitboard {
    pub pawns: BitB64,
    pub knights: BitB64,
    pub bishops: BitB64,
    pub rooks: BitB64,
    pub queens: BitB64,
    pub king: BitB64,
}

impl PlayerBitboard {
    pub fn empty() -> PlayerBitboard {
        PlayerBitboard {
            pawns: EMPTY_BOARD,
            knights: EMPTY_BOARD,
            bishops: EMPTY_BOARD,
            rooks: EMPTY_BOARD,
            queens: EMPTY_BOARD,
            king: EMPTY_BOARD,
        }
    }

    pub fn new(color: PlayerColor) -> PlayerBitboard {
        match color {
            PlayerColor::White => PlayerBitboard {
                pawns: u64::nth(A2)
                    | u64::nth(B2)
                    | u64::nth(C2)
                    | u64::nth(D2)
                    | u64::nth(E2)
                    | u64::nth(F2)
                    | u64::nth(G2)
                    | u64::nth(H2),
                knights: u64::nth(B1) | u64::nth(G1),
                bishops: u64::nth(C1) | u64::nth(F1),
                rooks: u64::nth(A1) | u64::nth(H1),
                queens: u64::nth(D1),
                king: u64::nth(E1),
            },
            PlayerColor::Black => PlayerBitboard {
                pawns: u64::nth(A7)
                    | u64::nth(B7)
                    | u64::nth(C7)
                    | u64::nth(D7)
                    | u64::nth(E7)
                    | u64::nth(F7)
                    | u64::nth(G7)
                    | u64::nth(H7),
                knights: u64::nth(B8) | u64::nth(G8),
                bishops: u64::nth(C8) | u64::nth(F8),
                rooks: u64::nth(A8) | u64::nth(H8),
                queens: u64::nth(D8),
                king: u64::nth(E8),
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
