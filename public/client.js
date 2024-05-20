import { numToRow, getSquareName } from "./common.js"

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


export async function IssuePossibleMovesReq(board, player_to_move) {
    console.log("player:", player_to_move)
    let uci_req = {
        p_to_move: player_to_move,
        board: EncodeBoard(board),
        req_type: "possible_moves",
        timeout: 3000,
    }
    console.log('issuing UCI req:');
    console.log(uci_req);
    let uci_response = await IssueUciReq(uci_req);
    console.log(uci_response);
    return uci_response.possible_moves;
}


export async function parsePossibleMoves(possibleMoves) {
    let temp = await possibleMoves;
    const moves = temp.split(',');
    const moveMap = new Map();

    for (const move of moves) {
        const from = move.split(':')[0];
        const to = move.split(':')[1];
        if (!moveMap.has(from)) {
            moveMap.set(from, []);
        }
        moveMap.get(from).push(to);
    }
    return moveMap;
}

async function IssueUciReq(uci_req) {
    const url = "http://127.0.0.1:9999";
    console.log('Sending request to engine..');
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
    console.log('Received valid JSON Response!!:');
    console.log(response_json);
    return response_json;
}

