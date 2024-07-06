use rocket::log::private::debug;

use super::bitboard::SpecialMoveType;
use super::zobrist::ZobristTable;
use crate::chess::bitboard::{
    BitArraySize, BitB64, BitboardMove, PlayerBitboard, EMPTY_BOARD, FULL_BOARD,
};
use crate::move_gen::internal::intersect;
use strum::IntoEnumIterator;

use crate::chess::{ChessPiece, PieceType, PlayerColor};
use crate::move_gen::{self, MoveGenOpts, MoveGenPerspective};
use crate::move_gen::{
    bishop::BishopBitboardMoveGenerator, king::KingBitboardMoveGenerator,
    knight::KnightBitboardMoveGenerator, merge_moves_map, pawn::PawnBitboardMoveGenerator,
    queen::QueenBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};
use crate::UciRequest;
use rand::{thread_rng, Rng};

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

enum PositionInfoMetadataBits {
    PlayerToMove,
}
#[derive(EnumIter)]
pub enum CastlingRightsBits {
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
    pub zobrist_hash: u64,
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
            zobrist_hash: 0,
        };
        result
    }
    pub fn pass_turn(&mut self) {
        // Flip bit 0 and 1. 3 = 1 + 2
        self.metadata ^= u8::nth(PositionInfoMetadataBits::PlayerToMove as u8);
        let table = ZobristTable::get();
        self.zobrist_hash ^= table.black_to_move;
    }
    pub fn enemy_player(&self) -> PlayerColor {
        if Self::white_to_move(&self) {
            PlayerColor::Black
        } else {
            PlayerColor::White
        }
    }

    pub fn player_to_move(&self) -> PlayerColor {
        if Self::white_to_move(&self) {
            PlayerColor::White
        } else {
            PlayerColor::Black
        }
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

    pub fn has_short_castling_rights(&self, color: PlayerColor) -> bool {
        true
        // match color {
        //     PlayerColor::White => {
        //         (self.castling_rights & u8::nth(CastlingRightsBits::WhiteShortCastlingRights as u8))
        //             != 0
        //     }
        //     PlayerColor::Black => {
        //         (self.castling_rights & u8::nth(CastlingRightsBits::BlackShortCastlingRights as u8))
        //             != 0
        //     }
        // }
    }
    pub fn has_long_castling_rights(&self, color: PlayerColor) -> bool {
        true
        // match color {
        //     PlayerColor::White => {
        //         (self.castling_rights & u8::nth(CastlingRightsBits::WhiteLongCastlingRights as u8))
        //             != 0
        //     }
        //     PlayerColor::Black => {
        //         (self.castling_rights & u8::nth(CastlingRightsBits::BlackLongCastlingRights as u8))
        //             != 0
        //     }
        // }
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

    pub fn get_raw_attacked_squares(&self, perspective: &MoveGenPerspective) -> BitB64 {
        let generators = vec![
            PawnBitboardMoveGenerator::get_raw_attacking_moves,
            KnightBitboardMoveGenerator::get_raw_attacking_moves,
            BishopBitboardMoveGenerator::get_raw_attacking_moves,
            RookBitboardMoveGenerator::get_raw_attacking_moves,
            QueenBitboardMoveGenerator::get_raw_attacking_moves,
            KingBitboardMoveGenerator::get_raw_attacking_moves,
        ];
        let mut result = EMPTY_BOARD;
        for generator in generators {
            result |= generator(
                &self,
                MoveGenOpts {
                    perspective: *perspective,
                },
            )
        }
        result
    }

    pub fn compute_zobrist_hash(&mut self) {
        let table = ZobristTable::get();
        self.position_info.zobrist_hash = 0;

        // Iterate over all squares
        let white_pieces = &self.white;
        let black_pieces = &self.black;

        let mut color_id: usize = 0;
        for colored_pieces in [white_pieces, black_pieces].iter() {
            for piece_type in PieceType::iter() {
                let mut piece_set = *colored_pieces.pieces(piece_type);
                while piece_set != EMPTY_BOARD {
                    let id: i8 = piece_set.trailing_zeros() as i8;
                    self.position_info.zobrist_hash ^=
                        table.table[piece_type as usize][color_id][id as usize];
                    piece_set ^= u64::nth(id as u8);
                }
            }
            color_id += 1;
        }
        // XOR with random values for castling rights
        if self
            .position_info
            .has_short_castling_rights(PlayerColor::White)
        {
            self.position_info.zobrist_hash ^=
                table.castling_rights[CastlingRightsBits::WhiteShortCastlingRights as usize];
        }
        if self
            .position_info
            .has_long_castling_rights(PlayerColor::White)
        {
            self.position_info.zobrist_hash ^=
                table.castling_rights[CastlingRightsBits::WhiteLongCastlingRights as usize];
        }
        if self
            .position_info
            .has_short_castling_rights(PlayerColor::Black)
        {
            self.position_info.zobrist_hash ^=
                table.castling_rights[CastlingRightsBits::BlackShortCastlingRights as usize];
        }
        if self
            .position_info
            .has_long_castling_rights(PlayerColor::Black)
        {
            self.position_info.zobrist_hash ^=
                table.castling_rights[CastlingRightsBits::BlackLongCastlingRights as usize];
        }

        // TODO implement enpassant.

        // XOR with random value for side to move
        if self.player_to_move() == PlayerColor::Black {
            self.position_info.zobrist_hash ^= table.black_to_move;
        }
    }

    pub fn player_to_move(&self) -> PlayerColor {
        self.position_info.player_to_move()
    }

    pub fn waiting_player(&self) -> PlayerColor {
        self.position_info.enemy_player()
    }

    pub fn mut_pieces_to_move(&mut self) -> &mut PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::Black => &mut self.black,
            PlayerColor::White => &mut self.white,
        }
    }

    pub fn pieces_to_move(&self) -> &PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::Black => &self.black,
            PlayerColor::White => &self.white,
        }
    }

    pub fn mut_enemy_pieces(&mut self) -> &mut PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::White => &mut self.black,
            PlayerColor::Black => &mut self.white,
        }
    }

    pub fn enemy_pieces(&self) -> &PlayerBitboard {
        match self.player_to_move() {
            PlayerColor::White => &self.black,
            PlayerColor::Black => &self.white,
        }
    }

    pub fn update_info(&mut self) {
        self.pass_turn();
    }
    // Returns the zobrist mutation to be applied to the position
    pub fn make_raw_bitboard_move(
        (from, to): (u8, u8),
        (moving_player, waiting_player): (PlayerColor, PlayerColor),
        (moving_pieces, waiting_pieces): (&mut PlayerBitboard, &mut PlayerBitboard),
        typpe: PieceType,
    ) -> u64 {
        let z_table = ZobristTable::get();
        let mut zobrist_mutation = 0;
        let to_sq = u64::nth(to);
        let moving_piece_set = moving_pieces.mut_pieces(typpe);
        *moving_piece_set ^= u64::nth(from);
        *moving_piece_set |= to_sq;
        zobrist_mutation ^= z_table.table[typpe as usize][moving_player as usize][from as usize];
        zobrist_mutation ^= z_table.table[typpe as usize][moving_player as usize][to as usize];

        let delete_enemy_pieces_mask = FULL_BOARD ^ to_sq;
        for piece_type in PieceType::iter() {
            let waiting_piece_set = waiting_pieces.mut_pieces(piece_type);
            if intersect(*waiting_piece_set, to_sq) {
                *waiting_piece_set &= delete_enemy_pieces_mask;
                zobrist_mutation ^=
                    z_table.table[piece_type as usize][waiting_player as usize][to as usize];
                break;
            }
        }

        zobrist_mutation
    }

    fn execute_promotion(&mut self, typpe: PieceType, to_sq: u64) -> u64 {
        let mut zobrist_mutation = 0;
        let z_table = ZobristTable::get();
        let piece_set = self.mut_pieces_to_move().mut_pieces(typpe);
        *piece_set ^= to_sq;
        self.mut_pieces_to_move().pawns ^= to_sq;
        zobrist_mutation ^=
            z_table.table[typpe as usize][self.player_to_move() as usize][to_sq as usize];
        zobrist_mutation ^=
            z_table.table[PieceType::Pawn as usize][self.waiting_player() as usize][to_sq as usize];
        zobrist_mutation
    }

    pub fn make_move(&self, mv: &BitboardMove, piece: ChessPiece) -> Position {
        // Remove the piece from its old position.
        // gets black or white bitboard
        let mut result = *self;
        let (ally_pieces, mut enemy_pieces) = match piece.color {
            PlayerColor::White => (&mut result.white, &mut result.black),
            PlayerColor::Black => (&mut result.black, &mut result.white),
        };
        result.position_info.zobrist_hash ^= Self::make_raw_bitboard_move(
            (mv.from, mv.to),
            (self.player_to_move(), self.waiting_player()),
            (ally_pieces, enemy_pieces),
            piece.typpe,
        );
        match mv.sp_move_type {
            SpecialMoveType::RegularMove => (),
            SpecialMoveType::ShortCastle => {
                result.position_info.zobrist_hash ^= Self::make_raw_bitboard_move(
                    move_gen::rook::get_rook_move_for_short_castle(piece.color),
                    (self.player_to_move(), self.waiting_player()),
                    (ally_pieces, enemy_pieces),
                    piece.typpe,
                );
            }
            SpecialMoveType::LongCastle => {
                result.position_info.zobrist_hash ^= Self::make_raw_bitboard_move(
                    move_gen::rook::get_rook_move_for_long_castle(piece.color),
                    (self.player_to_move(), self.waiting_player()),
                    (ally_pieces, enemy_pieces),
                    piece.typpe,
                );
            }
            SpecialMoveType::EnPassantLeft => todo!(),
            SpecialMoveType::EnPassantRight => todo!(),
            SpecialMoveType::PromotionToBishop => {
                let to_sq = u64::nth(mv.to);
                result.position_info.zobrist_hash ^=
                    result.execute_promotion(PieceType::Bishop, to_sq);
            }
            SpecialMoveType::PromotionToKnight => {
                let to_sq = u64::nth(mv.to);
                result.position_info.zobrist_hash ^=
                    result.execute_promotion(PieceType::Knight, to_sq);
            }
            SpecialMoveType::PromotionToRook => {
                let to_sq = u64::nth(mv.to);
                result.position_info.zobrist_hash ^=
                    result.execute_promotion(PieceType::Rook, to_sq);
            }
            SpecialMoveType::PromotionToQueen => {
                let to_sq = u64::nth(mv.to);
                result.position_info.zobrist_hash ^=
                    result.execute_promotion(PieceType::Queen, to_sq);
            }
        }
        // TODO(implement castling info updates.)
        result.update_info();
        // Return mutable reference to self to allow chaining calls.
        result
    }

    // Returns whether king of given |color| can be capturued.
    pub fn can_king_be_captured(&self, perspective: MoveGenPerspective) -> bool {
        let king_pieces = match perspective {
            MoveGenPerspective::MovingPlayer => self.pieces_to_move(),
            MoveGenPerspective::WaitingPlayer => &self.enemy_pieces(),
        };
        let attacked_squares_perspective = match perspective {
            MoveGenPerspective::MovingPlayer => MoveGenPerspective::WaitingPlayer,
            MoveGenPerspective::WaitingPlayer => MoveGenPerspective::MovingPlayer,
        };
        crate::move_gen::internal::intersect(
            king_pieces.king,
            self.get_raw_attacked_squares(&attacked_squares_perspective),
        )
    }

    pub fn move_puts_own_king_in_check(&self, mv: &BitboardMove, piece: ChessPiece) -> bool {
        let new = self.make_move(mv, piece);
        new.can_king_be_captured(MoveGenPerspective::WaitingPlayer)
    }

    pub fn legal_continuations(&self) -> MovesMap {
        let possible_moves_map = self.pseudolegal_continuations();
        let mut result = MovesMap::new();
        // For each square, we know if there's a piece in it pseudolegal moves.
        for (from_id, piece_and_moves) in possible_moves_map.iter() {
            let typpe = piece_and_moves.typpe;
            let moves_list = &piece_and_moves.moves;
            let mut legal_moves = Vec::new();
            for mv in moves_list.iter() {
                if !self.move_puts_own_king_in_check(
                    &mv,
                    ChessPiece {
                        typpe: typpe,
                        color: self.player_to_move(),
                    },
                ) {
                    legal_moves.push(*mv);
                }
            }
            if legal_moves.len() > 0 {
                result.insert(
                    *from_id,
                    PieceAndMoves {
                        typpe: typpe,
                        moves: legal_moves,
                    },
                );
            }
        }
        result
    }

    pub fn pseudolegal_continuations(&self) -> MovesMap {
        let mut result = MovesMap::new();

        let piece_generators = vec![
            PawnBitboardMoveGenerator::generate_moves,
            KnightBitboardMoveGenerator::generate_moves,
            BishopBitboardMoveGenerator::generate_moves,
            RookBitboardMoveGenerator::generate_moves,
            QueenBitboardMoveGenerator::generate_moves,
            KingBitboardMoveGenerator::generate_moves,
        ];

        for generate_moves in piece_generators.iter() {
            merge_moves_map(
                generate_moves(
                    &self,
                    MoveGenOpts {
                        perspective: MoveGenPerspective::MovingPlayer,
                    },
                ),
                &mut result,
            );
        }

        result
    }
}
