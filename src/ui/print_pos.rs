use crate::chess::bitboard::BitB64;
use crate::chess::position::Position;
use crate::chess::{ChessPiece, PieceType, PlayerColor};
use strum_macros::EnumIter; // 0.17.1
use strum::IntoEnumIterator;

fn piece_as_unicode(piece: ChessPiece) -> &'static str {
    match piece.color {
        PlayerColor::Black => match piece.typpe {
            PieceType::Pawn => "♟",
            PieceType::Knight => "♞",
            PieceType::Bishop => "♝",
            PieceType::Rook => "♜",
            PieceType::Queen => "♛",
            PieceType::King => "♚",
        },
        PlayerColor::White => match piece.typpe {
            PieceType::Pawn => "♙",
            PieceType::Knight => "♘",
            PieceType::Bishop => "♗",
            PieceType::Rook => "♖",
            PieceType::Queen => "♕",
            PieceType::King => "♔",
        },
    }
}

fn print_unicode_chess_position(position: &Position) {
    for rank in 0..8 {
        for file in 0..8 {
            let mut found = false;
            let square: BitB64 = 1u64 << (rank * 8 + file);
            let black_union: BitB64 = position.black_pieces().get_pieces();
            let black: BitB64 = position.black_pieces().get_pieces();
            let white: BitB64 = position.white_pieces().get_pieces();
            let white_union: BitB64 = position.white_pieces().get_pieces();
            for typpe in PieceType::iter() {
                if (black_union & square) != 0u64 {
                    print!(
                        "{}",
                        piece_as_unicode(ChessPiece {
                            typpe: typpe,
                            color: PlayerColor::Black
                        })
                    );
                    found = true
                } else if white_union & square != 0u64 {
                    print!(
                        "{}",
                        piece_as_unicode(ChessPiece {
                            typpe: typpe,
                            color: PlayerColor::White
                        })
                    );
                    found = true
                }
                if found {break}
            }
            print!(".");
        }
    }

    println!();
}
