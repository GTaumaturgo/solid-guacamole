use chess;
use move_gen;

pub struct PositionInfo {
    pub white_unused_en_passant: u8,
    pub black_unused_en_passant: u8,
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
            castling_rights: 0,
            metadata: 0,
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
            position_info: 0, // revise later.
        }
    }
    pub fn white_to_move(&self) -> bool {
        self.position_info & !(1u64 << 20)
    }
    pub fn black_to_move(&self) -> bool {
        self.position_info & (1u64 << 20)
    }
    pub fn legal_continuations(&self) -> Vec<BitboardMove> {
        {}
        // let ally_bitboard = if self.white_to_move() {
        //     self.white.get_pieces()
        // } else {
        //     self.black.get_pieces()
        // };
        // let enemy_bitboard = if self.white_to_move() {
        //     self.black.get_pieces()
        // } else {
        //     self.white.get_pieces()
        // };
    }
    pub fn pseudolegal_continuations(&self) -> Vec<BitboardMove> {
        {}
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
