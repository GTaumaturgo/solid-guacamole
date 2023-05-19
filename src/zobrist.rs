use crate::common::{Color, PieceType};
use rand::Rng;

const NUM_SQUARES: usize = 64;

pub struct Zobrist {
    // The table is indexed by color, piece type, and square.
    table: [[[u64; NUM_SQUARES]; PieceType::NumPieceTypes as usize]; Color::NumColors as usize],
    white_to_move: u64,
    // Add more fields for castling rights and en passant if needed.
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut table: [[[u64; NUM_SQUARES]; PieceType::NumPieceTypes as usize];
            Color::NumColors as usize];

        for piece_color in 0..(Color::NumColors as usize) {
            for piece_type in 0..(PieceType::NumPieceTypes as usize) {
                for square in 0..NUM_SQUARES {
                    table[piece_color][piece_type][square] = rng.gen::<u64>();
                }
            }
        }

        let white_to_move = rng.gen::<u64>();
        // Generate random values for castling rights and en passant if needed.

        Zobrist {
            table,
            white_to_move,
        }
    }

    pub fn hash(&self, board: &crate::bitboard::Chessboard, color_to_move: Color) -> u64 {
        let mut hash = 0u64;

        for square in 0..NUM_SQUARES {
            if let Some((piece, color)) = board.get_piece_and_color(square) {
                let piece_type_index = piece as usize + (if color == Color::Black { 6 } else { 0 });
                hash ^= self.table[color as usize][piece_type_index][square];
            }
        }

        if color_to_move == Color::White {
            hash ^= self.white_to_move;
        }

        // XOR the hash with castling rights and en passant values if needed.

        hash
    }
}
