use rocket::log::private::debug;

use crate::chess::bitboard::{
    BitArraySize, BitB64, BitboardMove, PlayerBitboard, EMPTY_BOARD, FULL_BOARD,
};
use crate::chess::{ChessPiece, PieceType, PlayerColor};
use crate::move_gen::{
    bishop::BishopBitboardMoveGenerator, king::KingBitboardMoveGenerator,
    knight::KnightBitboardMoveGenerator, merge_moves_map, pawn::PawnBitboardMoveGenerator,
    queen::QueenBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};
use crate::UciRequest;

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
        self.metadata ^= u8::nth(PositionInfoMetadataBits::PlayerToMove as u8);
    }

    // Mutates metadata setting the player to move. Returns the metadata to be consumed optionally if convenient.
    pub fn set_player_to_move(&mut self, color: PlayerColor) -> u8 {
        let result = match color {
            PlayerColor::White => {
                u8::disable_nth(self.metadata, PositionInfoMetadataBits::PlayerToMove as u8)
            }
            PlayerColor::Black => {
                u8::enable_nth(self.metadata, PositionInfoMetadataBits::PlayerToMove as u8)
            }
        };
        self.metadata = result;
        self.metadata
    }

    pub fn white_to_move(&self) -> bool {
        let white_as_u8 = PlayerColor::White as u8;
        self.metadata & u8::nth(PositionInfoMetadataBits::PlayerToMove as u8) == white_as_u8
    }

    pub fn black_to_move(&self) -> bool {
        !self.white_to_move()
    }

    pub fn short_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => {
                (self.castling_rights & u8::nth(CastlingRightsBits::WhiteShortCastlingRights as u8))
                    != 0
            }
            PlayerColor::Black => {
                (self.castling_rights & u8::nth(CastlingRightsBits::BlackShortCastlingRights as u8))
                    != 0
            }
        }
    }
    pub fn long_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => {
                (self.castling_rights & u8::nth(CastlingRightsBits::WhiteLongCastlingRights as u8))
                    != 0
            }
            PlayerColor::Black => {
                (self.castling_rights & u8::nth(CastlingRightsBits::BlackLongCastlingRights as u8))
                    != 0
            }
        }
    }
}

// Bitboard representation of a chess position.
#[derive(Clone, Copy)]
pub struct Position {
    pub white: PlayerBitboard,
    pub black: PlayerBitboard,
    pub position_info: PositionInfo,
}

impl Position {
    pub fn new() -> Position {
        Position {
            white: PlayerBitboard::new(PlayerColor::White),
            black: PlayerBitboard::new(PlayerColor::Black),
            position_info: PositionInfo::new(),
        }
    }

    pub fn decode_pieces(board: &String) -> (PlayerBitboard, PlayerBitboard) {
        let mut white = PlayerBitboard::empty();
        let mut black = PlayerBitboard::empty();

        for (i, ch) in board.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            let bitb = match ch {
                'k' => &mut white.king,
                'K' => &mut black.king,
                'q' => &mut white.queens,
                'Q' => &mut black.queens,
                'r' => &mut white.rooks,
                'R' => &mut black.rooks,
                'b' => &mut white.bishops,
                'B' => &mut black.bishops,
                'n' => &mut white.knights,
                'N' => &mut black.knights,
                'p' => &mut white.pawns,
                'P' => &mut black.pawns,
                _ => todo!(),
            };
            *bitb |= u64::nth(i as u8);
        }
        // todo
        (white, black)
    }

    pub fn decode_position_info(uci_req: &UciRequest) -> PositionInfo {
        let mut result = PositionInfo::new();

        result.set_player_to_move(match uci_req.p_to_move.as_ref() {
            "B" => PlayerColor::Black,
            "W" => PlayerColor::White,
            _ => todo!(),
        });
        result
    }

    pub fn from_uci(uci_req: &UciRequest) -> Position {
        //fill position info
        let pos_info = Self::decode_position_info(uci_req);
        // fill bitboards
        let (white, black) = Self::decode_pieces(&uci_req.board);

        Position {
            white: white,
            black: black,
            position_info: pos_info,
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

    pub fn enemy_player(&self) -> PlayerColor {
        if self.position_info.white_to_move() {
            PlayerColor::Black
        } else {
            PlayerColor::White
        }
    }

    pub fn pieces_to_move(&self) -> &PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::Black => &self.black,
            PlayerColor::White => &self.white,
        }
    }

    pub fn enemy_pieces(&self) -> &PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::White => &self.white,
            PlayerColor::Black => &self.black,
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
        //  &= !u64::nth(mv.from)
        let pieces = pieces_to_move.mut_pieces(piece.typpe);
        *pieces &= FULL_BOARD & u64::nth(mv.from);
        *pieces |= u64::nth(mv.to);

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
            let mut legal_move_set = EMPTY_BOARD;
            while move_set != EMPTY_BOARD {
                let zeros = move_set.trailing_zeros() as u8;
                let bitb_move = BitboardMove {
                    from: *from_id,
                    to: zeros,
                };
                if !self.is_check_after_move(
                    &bitb_move,
                    ChessPiece {
                        typpe: typpe,
                        color: self.player_to_move(),
                    },
                ) {
                    legal_move_set |= u64::nth(zeros);
                }
                move_set ^= u64::nth(zeros);
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
        merge_moves_map(
            PawnBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );
        merge_moves_map(
            KnightBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );
        merge_moves_map(
            BishopBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );
        merge_moves_map(
            RookBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );
        merge_moves_map(
            QueenBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );
        merge_moves_map(
            KingBitboardMoveGenerator::generate_moves(&self),
            &mut result,
        );

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
