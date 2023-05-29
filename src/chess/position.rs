use crate::chess::{PlayerColor, ChessPiece, PieceType};
use crate::chess::bitboard::{Bitboard,BitboardMove,BitB64};

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
}

impl PositionInfo {
    pub fn new() -> PositionInfo {
        PositionInfo {
            white_unused_en_passant: 0,
            black_unused_en_passant: 0,
            white_usable_en_passant: 0,
            black_usable_en_passant: 0,
            castling_rights: 0,
            metadata: 0,
        }
    }
    pub fn pass_turn(&mut self) {
        // Flip bit 0 and 1. 3 = 1 + 2
        self.metadata ^= 3
    }

    pub fn white_to_move(&self) -> bool {
        return (self.metadata & (1u8 << 0)) != 0;
    }

    pub fn set_white_to_move(&mut self) {
        self.metadata |= 1u8 << 0;
    }

    pub fn set_black_to_move(&mut self) {
        self.metadata &= !1u8 << 0;
    }

    pub fn short_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => (self.castling_rights & (1u8 << 0)) != 0,
            PlayerColor::Black => (self.castling_rights & (1u8 << 1)) != 0,
        }
    }
    pub fn long_castling_allowed(&self, color: PlayerColor) -> bool {
        match color {
            PlayerColor::White => (self.castling_rights & (1u8 << 2)) != 0,
            PlayerColor::Black => (self.castling_rights & (1u8 << 3)) != 0,
        }
    }
}

// Bitboard representation of a chess position.
pub struct Position {
    pub white: Bitboard,
    pub black: Bitboard,
    pub position_info: PositionInfo,
}

impl Position {
    pub fn new() -> Position {
        Position {
            white: Bitboard::new(),
            black: Bitboard::new(),
            position_info: PositionInfo::new(),
        }
    }
    // pub fn white_to_move(&self) -> bool {
    //     self.position_info & !1u64 << 20
    // }
    // pub fn black_to_move(&self) -> bool {
    //     self.position_info & 1u64 << 20
    // }

    pub fn pass_turn(&mut self) -> () {
        self.position_info.pass_turn();
    }

    pub fn make_move(&mut self, mv: &BitboardMove, piece: ChessPiece) -> () {
        // Remove the piece from its old position.
        // gets black or white bitboard

        let bitboard = match piece.color {
            PlayerColor::White => &mut self.white,
            PlayerColor::Black => &mut self.black,
        };
        // remove piece from its old pos
        match piece.typpe {
            PieceType::Pawn => bitboard.pawns &= !(1u64 << mv.from),
            PieceType::Knight => bitboard.knights &= !(1u64 << mv.from),
            PieceType::Bishop => bitboard.bishops &= !(1u64 << mv.from),
            PieceType::Rook => bitboard.rooks &= !(1u64 << mv.from),
            PieceType::Queen => bitboard.queens &= !(1u64 << mv.from),
            PieceType::King => bitboard.king &= !(1u64 << mv.from),
        };

        // move to new pos
        match piece.typpe {
            PieceType::Pawn => bitboard.pawns |= 1u64 << mv.from,
            PieceType::Knight => bitboard.knights |= 1u64 << mv.from,
            PieceType::Bishop => bitboard.bishops |= 1u64 << mv.from,
            PieceType::Rook => bitboard.rooks |= 1u64 << mv.from,
            PieceType::Queen => bitboard.queens |= 1u64 << mv.from,
            PieceType::King => bitboard.king |= 1u64 << mv.from,
        };

        // if piece.typpe == PieceType::Pawn && abs(mv.from - mv.to) == 16 {
        //     // update used
        //     // update usable
        // }
        

        // Update the castling rights.
        
    }

    pub fn is_check(&self, color: PlayerColor) -> bool {
        // let own_pieces = 
        // let enemy_pieces = 
        return false;
    }

    pub fn white_pieces(&self) -> &Bitboard {
        &self.white
    }
    pub fn mut_white_pieces(&mut self) -> &mut Bitboard {
        &mut self.white
    }

    pub fn black_pieces(&self) -> &Bitboard {
        &self.black
    }
    pub fn mut_black_pieces(&mut self) -> &mut Bitboard {
        &mut self.black
    }

    pub fn is_check_after_move(&self, mv: &BitboardMove, piece: ChessPiece) -> bool {
        // Make the mv.
        // self.make_move(mv, piece);

        // // Check if the king is in check.
        // let is_check = self.is_check(mv, piece.color);

        // // Undo the mv.
        // self.undo_move(mv);

        // // Return the result.
        // return is_check;
        return false;
    }
    pub fn legal_continuations(&self) -> Vec<BitboardMove> {
        // Get the list of all possible moves.
        let possible_moves = self.pseudolegal_continuations();

        // Filter out the moves that put the king in check.
        let mut legal_moves = Vec::new();
        // for mv in possible_moves {
        //     if !self.is_check_after_move(mv) {
        //         legal_moves.push(mv);
        //     }
        // }

        // Return the list of legal moves.
        return legal_moves;
    }

    pub fn pseudolegal_continuations(&self) -> Vec<BitboardMove> {
        // Iterate over all of the pieces on the board.
        let pseudolegal_moves = Vec::new();
        // for piece in self.pieces.iter() {
        //     // Get the piece's possible moves.
        //     let possible_moves = piece.possible_moves();

        //     // For each possible move, add it to the list of pseudolegal moves.
        //     for mv in possible_moves {
        //         pseudolegal_moves.push(mv);
        //     }
        // }

        // Return the list of pseudolegal moves.
        return pseudolegal_moves;
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
