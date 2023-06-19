use crate::{
    bitb,
    chess::{position::{Position, self}, bitboard::{BitB64, Bitboard}},
    UciRequest, UciResponse,
};

pub fn HandlePossibleMovesRequest(uci_req: &UciRequest) -> UciResponse {
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
    let continuations = position.legal_continuations();
    let mut possible_moves: String = "".to_owned();
    // for bitb_move in continuations.iter() {
    //     possible_moves += format!("{},{}", bitb_move.from, bitb_move.to).as_ref();
    // }
    println!("Computed possible moves: [{}]", possible_moves);
    UciResponse {
        best_moves: "".to_string(),
        possible_moves: "A1:B2,B2:C3,B2:C4".to_string(),
    }
}
