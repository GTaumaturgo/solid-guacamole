use super::MovesMap;
use crate::chess::{
    bitboard::{BitArraySize, BitB64, PlayerBitboard},
    position::Position,
};
pub fn is_inside_board(sq_id: i8) -> bool {
    0 <= sq_id && sq_id < 64
}

pub fn intersect(bitb_a: BitB64, bitb_b: BitB64) -> bool {
    bitb_a & bitb_b != 0
}

// Useful for bishop and rook.
pub fn try_generate_move_in_direction(
    sq_id: i8,
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    is_direction_blocked: &mut bool,
    cur_piece: &mut u64,
) -> () {
    if !(*is_direction_blocked) && is_inside_board(sq_id) {
        let sq_in_direction = u64::nth(sq_id as u8);
        if intersect(sq_in_direction, ally_pieces.all_pieces()) {
            *is_direction_blocked = true;
        } else if intersect(sq_in_direction, enemy_pieces.all_pieces()) {
            *cur_piece |= sq_in_direction;
            *is_direction_blocked = true;
        }
    }
    ()
}

pub mod test_utils {
    use crate::chess::position::Position;

    pub fn get_initial_position() -> super::Position {
        Position::new()
    }
}
