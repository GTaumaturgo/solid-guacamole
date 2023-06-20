use rocket::log::private::debug;

use crate::chess::bitboard::{BitB64, Bitboard, BitboardMove};
use crate::chess::{ChessPiece, PieceType, PlayerColor};
use crate::move_gen::{
    bishop, king, knight, merge_moves_map, pawn, queen, rook, MovesMap, PieceAndMoves,
};
use crate::RUNTIME;
use crate::{bitb, bitb16, bitb32, bitb8};

use super::bitboard::{self, full_board, empty_board};

enum PositionInfoMetadataBits {
    PlayerToMove,
}

enum CastlingRightsBits {
    WhiteShortCastlingRights,
    WhiteLongCastlingRights,
    BlackShortCastlingRights,
    BlackLongCastlingRights,
}

#[derive(Clone, Copy)]
pub struct PositionInfo {
    pub white_unused_en_passant: u8,
    pub black_unused_en_passant: u8,
    pub white_usable_en_passant: u8,
    pub black_usable_en_passant: u8,
    // Castling rights.
    // bit 0: white short castling rights.
    // bit 1: white long castling rights.
    // bit 2: black short castling rights.
    // bit 3: black long castling rights.
    pub castling_rights: u8,
    // Other info like player to move.
    // bit 0: white to move.
    // bit 1: black to move.
    // bit 2: unused.
    // ...
    pub metadata: u8,
    // pub moves_map: Option<MovesMap>,
}

impl PositionInfo {
    pub fn new() -> PositionInfo {
        let mut result = PositionInfo {
            white_unused_en_passant: 0,
            black_unused_en_passant: 0,
            white_usable_en_passant: 0,
            black_usable_en_passant: 0,
            castling_rights: 0,
            metadata: 0,
        };
        result
    }
    pub fn pass_turn(&mut self) {
        // Flip bit 0 and 1. 3 = 1 + 2
        self.metadata ^= bitb8!(PositionInfoMetadataBits::PlayerToMove);
    }

    pub fn white_to_move(&self) -> bool {
        self.metadata & bitb8!(PositionInfoMetadataBits::PlayerToMove) == 0
    }

    pub fn black_to_move(&self) -> bool {
        !self.white_to_move()
    }

    pub fn short_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => {
                (self.castling_rights & bitb8!(CastlingRightsBits::WhiteShortCastlingRights)) != 0
            }
            PlayerColor::Black => {
                (self.castling_rights & bitb8!(CastlingRightsBits::BlackShortCastlingRights)) != 0
            }
        }
    }
    pub fn long_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => {
                (self.castling_rights & bitb8!(CastlingRightsBits::WhiteLongCastlingRights)) != 0
            }
            PlayerColor::Black => {
                (self.castling_rights & bitb8!(CastlingRightsBits::BlackLongCastlingRights)) != 0
            }
        }
    }
}

// Bitboard representation of a chess position.
#[derive(Clone, Copy)]
pub struct Position {
    pub white: Bitboard,
    pub black: Bitboard,
    pub position_info: PositionInfo,
}

impl Position {
    pub fn new() -> Position {
        Position {
            white: Bitboard::new(PlayerColor::White),
            black: Bitboard::new(PlayerColor::Black),
            position_info: PositionInfo::new(),
        }
    }

    pub fn pass_turn(&mut self) -> () {
        self.position_info.pass_turn();
    }

    pub fn player_to_move(&self) -> PlayerColor {
        if self.position_info.white_to_move() {
            PlayerColor::White
        } else {
            PlayerColor::Black
        }
    }

    pub fn pieces_to_move(&self) -> &Bitboard {
        match self.player_to_move() {
            PlayerColor::Black => &self.black,
            PlayerColor::White => &self.white,
        }
    }

    pub fn update_info(&mut self) {
        self.pass_turn();
    }

    pub fn make_move(&mut self, mv: &BitboardMove, piece: ChessPiece) -> &mut Self {
        // Remove the piece from its old position.
        // gets black or white bitboard

        let pieces_to_move = match piece.color {
            PlayerColor::White => &mut self.white,
            PlayerColor::Black => &mut self.black,
        };
        // remove piece from its old pos
        //  &= !bitb!(mv.from)
        let pieces = pieces_to_move.mut_pieces(piece.typpe);
        *pieces &= full_board & bitb!(mv.from);
        *pieces |= bitb!(mv.to);

        // TODO(implement castling info updates.)
        self.update_info();
        // Return mutable reference to self to allow chaining calls.
        self
    }

    // Returns whether king of given |color| is in check.
    pub fn is_check(&self, color: PlayerColor) -> bool {
        // let own_pieces =
        // let enemy_pieces =
        return false;
    }

    pub fn is_check_after_move(&mut self, mv: &BitboardMove, piece: ChessPiece) -> bool {
        // Make the mv.
        let old = *self;
        self.make_move(mv, piece);
        let is_check = self.is_check(PlayerColor::other(piece.color));
        *self = old;
        return is_check;
    }

    pub fn legal_continuations(&mut self) -> MovesMap {
        let possible_moves_map = self.pseudolegal_continuations();
        let mut result = MovesMap::new();
        // For each square, we know if there's a piece in it pseudolegal moves.
        for (from_id, piece_and_moves) in possible_moves_map.iter() {
            let typpe = piece_and_moves.typpe;
            let mut move_set = piece_and_moves.moves;
            let mut legal_move_set = bitboard::empty_board;
            while move_set != empty_board {
                let zeros = move_set.trailing_zeros();
                // rightmost_one = bitb!(zeros + 1);
                let bitb_move = BitboardMove {
                    from: *from_id,
                    to: (zeros + 1) as u8,
                };
                if !self.is_check_after_move(
                    &bitb_move,
                    ChessPiece {
                        typpe: typpe,
                        color: self.player_to_move(),
                    },
                ) {
                    println!(
                        "adding move as its not checks: from: {} to: {}",
                        from_id,
                        zeros + 1
                    );
                    legal_move_set |= bitb!(zeros + 1);
                }
                move_set ^= bitb!(zeros + 1);
                break;
            }
            result.insert(
                *from_id,
                PieceAndMoves {
                    typpe: typpe,
                    moves: legal_move_set,
                },
            );
        }
        result
    }

    pub fn pseudolegal_continuations(&self) -> MovesMap {
        // Iterate over all of the pieces on the board.
        let mut result = MovesMap::new();

        let pieces = self.pieces_to_move();

        merge_moves_map(pawn::generate_moves(&self, pieces.pawns), &mut result);
        merge_moves_map(knight::generate_moves(pieces.knights), &mut result);
        merge_moves_map(bishop::generate_moves(pieces.bishops), &mut result);
        merge_moves_map(rook::generate_moves(pieces.rooks), &mut result);
        merge_moves_map(queen::generate_moves(pieces.queens), &mut result);
        merge_moves_map(king::generate_moves(pieces.king), &mut result);

        println!("pseudolegal!");
        result
    }
}
pub struct PositionScore {
    pub score: i32,
    // 0 by default. Different than 0 means that the position is a mate in x moves.
    pub mate_in: u8,
    // bit 0: stalemate.
    // bit 1: checkmate for white.
    // bit 2: checkmate for black.
    // bit 3: white_king_in_check.
    // bit 4: black_king_in_check.
    pub metadata: u8,
    // Bitboard of pinned pieces.
    pub pinned_pieces: u64,
}
pub struct ScoredPosition {
    pub position: Position,
    pub score: PositionScore,
}
