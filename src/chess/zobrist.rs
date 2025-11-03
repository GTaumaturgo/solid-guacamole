use rand::{thread_rng, Rng};
use std::sync::Once;
use strum::IntoEnumIterator;

use super::{PieceType, PlayerColor}; // Make sure you have this for iterating over your PieceType/PlayerColor enums

static mut ZOBRIST_TABLE: Option<ZobristTable> = None;
static INIT: Once = Once::new();

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ZobristHash(u64); // Newtype wrapper around u64

// impl ZobristHash {
//     pub fn new(hash: u64) -> Self {
//         Self(hash)
//     }
// }

// // Implement Hash trait for ZobristHash
// impl Hash for ZobristHash {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.hash(state); // Hash the underlying u64 value
//     }
// }

pub struct ZobristTable {
    pub table: [[[u64; 64]; 2]; 6], // PieceType * PlayerColor * Square
    pub castling_rights: [u64; 4],  // Castling rights
    // en_passant: [u64; 64],       // En passant square (file)
    pub black_to_move: u64, // Side to move
}

impl ZobristTable {
    pub fn get() -> &'static ZobristTable {
        unsafe {
            INIT.call_once(|| {
                ZOBRIST_TABLE = Some(ZobristTable::new());
            });
            ZOBRIST_TABLE.as_ref().unwrap()
        }
    }

    fn new() -> Self {
        let mut rng = thread_rng();
        let mut table = [[[0; 64]; 2]; 6];
        let mut castling_rights = [0; 4];
        // let mut en_passant = [0; 64];
        let black_to_move = rng.gen::<u64>();

        for piece_type in PieceType::iter() {
            for color in PlayerColor::iter() {
                for square in 0..64 {
                    table[piece_type as usize][color as usize][square] = rng.gen::<u64>();
                }
            }
        }

        for kind in super::position::CastlingRightsBits::iter() {
            castling_rights[kind as usize] = rng.gen::<u64>();
        }

        // for i in 0..64 {
        //     en_passant[i] = rng.gen::<u64>();
        // }

        Self {
            table,
            castling_rights,
            // en_passant,
            black_to_move,
        }
    }
}
