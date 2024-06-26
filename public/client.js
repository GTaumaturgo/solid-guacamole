import { numToRow, getSquareName } from "./common.js"

export const kPromotionMoveMapEntry = 'promotion';


class UciRequest {
    constructor(board, req_type) {
        this.board = EncodeBoard(board);
        this.req_type = req_type;
        this.timeout = 3000;
    }
}

// Encodes a chessboard into a string.
function EncodeBoard(board) {
    console.log(board);
    let output = "";
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            output += board.internal_state[i][j];
        }
    }
    return output;
}




// export function GetInitialSquareForKing()

export function GetLongClastingSquareforKing(from) {
    if (from == 'E1') {
        return 'C1';
    } else if (from == 'E8') {
        return 'C8';
    } else {
        console.log('ERROR: Invalid State in GetLongClastingSquareforKing');
        return null;
    }
}

// function GetLongCastlingSquareForRook(king_from) {
//     if (from == 'E1') {
//         return 'C1';
//     } else if (from == 'E8') {
//         return 'C8';
//     } else {
//         console.log('ERROR: Invalid State in GetLongClastingSquareforKing');
//         return null;
//     }
// }

// function GetLongCastlingSquareForRook(king_from) {
//     return king_from == 'E1' ? 'D1' : 'D8';
// }

export function GetShortClastingSquareforKing(from) {
    if (from == 'E1') {
        return 'G1';
    } else if (from == 'E8') {
        return 'G8';
    } else {
        console.log('ERROR: Invalid State in GetShortClastingSquareforKing');
        return null;
    }
}

// function GetShortCastlingSquareForRook(king_from) {
//     return king_from == 'E1' ? 'F1' : 'F8';
// }


export async function parsePossibleMoves(possibleMoves) {
    let temp = await possibleMoves;
    const moves = temp.split(',');
    const moveMap = new Map();

    for (const move of moves) {
        const from = move.split(':')[0];
        const to = move.split(':')[1];
        // Initialize if not.
        if (!moveMap.has(from)) {
            moveMap.set(from, []);
        }
        if (!moveMap.has(kPromotionMoveMapEntry)) {
            moveMap.set(kPromotionMoveMapEntry, new Set());
        }

        // console.log('Checking special moves:');
        if (to == 'O-O') {
            moveMap.get(from).push(GetShortClastingSquareforKing(from));
        } else if (to == 'O-O-O') {
            moveMap.get(from).push(GetLongCastlingSquareForKing(from));
        } else if (to.includes('+=')) {
            const to_sq = to.split('+=')[0];
            moveMap.get(from).push(to_sq);
            moveMap.get(kPromotionMoveMapEntry).add(to_sq);
        } else {
            moveMap.get(from).push(to);
        }
    }
    return moveMap;
}

export async function IssuePossibleMovesReq(board, player_to_move) {
    let uci_req = {
        p_to_move: player_to_move,
        board: EncodeBoard(board),
        req_type: "possible_moves",
        timeout: 3000,
    }
    let uci_response = await IssueUciReq(uci_req);
    return uci_response.possible_moves;
}

export async function IssuePositionEvalReq(board, player_to_move) {
    let uci_req = {
        p_to_move: player_to_move,
        board: EncodeBoard(board),
        req_type: "pos_eval",
        timeout: 3000,
    }
    let uci_response = await IssueUciReq(uci_req);
    return uci_response.pos_score;
}

async function IssueUciReq(uci_req) {
    const url = "http://127.0.0.1:9999";
    console.log('Sending request to Server..');
    console.log(uci_req);

    let response = await fetch(url, {
        method: "POST",
        mode: "cors",
        body: JSON.stringify(uci_req),
        headers: {
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*",
        }
    });
    let response_json = await (response.json());

    console.log('JSON Response from server!!:');
    console.log(response_json);
    return response_json;
}

