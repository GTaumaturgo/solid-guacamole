use crate::{
    bitb,
    chess::{ChessPiece,{
        bitboard::{BitB64, Bitboard, full_board},
        position::{self, Position},
    }},
    UciRequest, UciResponse, move_gen::PieceAndMoves,
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
    print!("{}", uci_req.board);
    
    let mut position = Position::new();

    for (i, char) in uci_req.board.chars().enumerate() {
        let cur_sq = bitb!(i);
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
        let mut cur_piece_moves = piece_n_moves.moves;
        while cur_piece_moves != 0 {
            let zeros = cur_piece_moves.trailing_zeros() as u8;
            println!("{}:{}", sq_id_to_name(*sq_id), sq_id_to_name(zeros));
            possible_moves += format!("{}:{},", sq_id_to_name(*sq_id), sq_id_to_name(zeros)).as_ref(); 
            cur_piece_moves ^= bitb!(zeros);
        }
    }
    
    possible_moves.pop();
    println!("Computed possible moves: [{}]", possible_moves);
    UciResponse {
        best_moves: "".to_string(),
        possible_moves: possible_moves,
        // possible_moves: "A2:A3,A2:A4,B2:B3,B2:B4,C2:C3,C2:C4,D2:D3,D2:D4,E2:E3,E2:E4,F2:F3,F2:F4,G2:G3,G2:G4,H2:H3,H2:H4".to_string(),
    }
}
