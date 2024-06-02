use super::MovesMap;
use crate::chess::{
    bitboard::{BitArraySize, BitB64, PlayerBitboard},
    position::Position,
};

pub fn bounded(val: i8, min: i8, max: i8) -> bool {
    min <= val && val <= max
}

pub fn get_i_from_sq_id(sq_id: i8) -> i8 {
    sq_id / 8
}

pub fn get_j_from_sq_id(sq_id: i8) -> i8 {
    sq_id % 8
}

pub fn get_ij_from_sq_id(sq_id: i8) -> (i8, i8) {
    (get_i_from_sq_id(sq_id), get_j_from_sq_id(sq_id))
}

pub fn get_sq_id_from_ij(i: i8, j: i8) -> i8 {
    i * 8 + j
}

pub fn is_inside_board(i: i8, j: i8) -> bool {
    bounded(i, 0, 7) && bounded(j, 0, 7)
}

pub fn intersect(bitb_a: BitB64, bitb_b: BitB64) -> bool {
    bitb_a & bitb_b != 0
}

// Useful for bishop and rook.
pub fn try_generate_move_in_direction(
    ij: (i8, i8),
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    is_direction_blocked: &mut bool,
    cur_piece_moves: &mut u64,
) -> bool {
    if !is_inside_board(ij.0, ij.1) {
        *is_direction_blocked = true;
        return false;
    }
    let sq_bitb = u64::nth(get_sq_id_from_ij(ij.0, ij.1) as u8);
    if *is_direction_blocked {
        false
    } else {
        if intersect(sq_bitb, ally_pieces.all_pieces()) {
            *is_direction_blocked = true;
            false
        } else if intersect(sq_bitb, enemy_pieces.all_pieces()) {
            *cur_piece_moves |= sq_bitb;
            *is_direction_blocked = true;
            true
        } else {
            *cur_piece_moves |= sq_bitb;
            true
        }
    }
}

pub mod test_utils {
    use crate::chess::position::Position;

    pub fn get_initial_position() -> super::Position {
        Position::new()
    }
}
