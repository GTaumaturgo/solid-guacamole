use crate::{
    chess::{
        bitboard::{BitArraySize, BitB64, PlayerBitboard, FULL_BOARD},
        position::Position,
    },
    move_gen::PieceAndMoves,
    UciRequest, UciResponse,
};

use std::str::from_utf8;

pub fn row_to_str(row: u8) -> String {
    (('0' as u8 + row + 1) as char).to_string()
}

pub fn col_to_str(col: u8) -> &'static str {
    match col {
        0 => "A",
        1 => "B",
        2 => "C",
        3 => "D",
        4 => "E",
        5 => "F",
        6 => "G",
        7 => "H",
        8_u8..=u8::MAX => todo!(),
    }
}

pub fn sq_id_to_name(sq_id: u8) -> String {
    let row = sq_id / 8;
    let col = sq_id % 8;
    format!("{}{}", col_to_str(col), row_to_str(row))
}

pub fn handle_possible_moves_request(uci_req: &UciRequest) -> UciResponse {
    println!("possible moves request");
    println!("Received board from UCI Req");
    println!("{}", uci_req.board);
    println!("Received p_to_move:");
    println!("{}", uci_req.p_to_move);

    let mut position = Position::from_uci(uci_req);

    for (i, char) in uci_req.board.chars().enumerate() {
        let cur_sq = u64::nth(i as u8);
        match char {
            'R' => position.black.rooks |= cur_sq,
            'N' => position.black.knights |= cur_sq,
            'B' => position.black.bishops |= cur_sq,
            'Q' => position.black.queens |= cur_sq,
            'K' => position.black.king |= cur_sq,
            'P' => position.black.pawns |= cur_sq,
            'r' => position.white.rooks |= cur_sq,
            'n' => position.white.knights |= cur_sq,
            'b' => position.white.bishops |= cur_sq,
            'q' => position.white.queens |= cur_sq,
            'k' => position.white.king |= cur_sq,
            'p' => position.white.pawns |= cur_sq,
            _ => (),
        }
    }
    let continuations_map = position.legal_continuations();
    let mut possible_moves: String = "".to_owned();
    for (sq_id, piece_n_moves) in continuations_map.iter() {
        let cur_piece_moves = &piece_n_moves.moves;
        for mv in piece_n_moves.moves.iter() {
            println!("{}:{}", sq_id_to_name(*sq_id), sq_id_to_name(mv.to));
            possible_moves +=
                format!("{}:{},", sq_id_to_name(*sq_id), sq_id_to_name(mv.to)).as_ref();
        }
    }

    possible_moves.pop();
    println!("Computed possible moves: [{}]", possible_moves);
    UciResponse {
        best_moves: "".to_string(),
        possible_moves: possible_moves,
    }
}
