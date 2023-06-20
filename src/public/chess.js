
class Board {
    constructor() {
        this.board = [
            ["white_rook", "white_knight", "white_bishop", "white_queen", "white_king", "white_bishop", "white_knight", "white_rook"],
            ["white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn"],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn"],
            ["black_rook", "black_knight", "black_bishop", "black_queen", "black_king", "black_bishop", "black_knight", "black_rook"],
        ];
    }

    setPiece(piece, row, col) {
        this.board[row][col] = piece;
    }

    getPiece(row, col) {
        return this.board[row][col];
    }

    setMoveMap(movesMap) {
        this.movesMap = movesMap;
    }
}


class Controller {

    constructor() {
        this.boards = [new Board()];
        let move_map = parsePossibleMoves(IssuePossibleMovesReq(this.boards[0]));
        this.boards[0].setMoveMap(move_map);
        this.selectedSquare = null;
        this.flipped = true;
    }

    flip_board() {
        this.flipped = !this.flipped;
    }
    async SelectSquare(row, column) {
        // If select the same square we unselect instead.
        let square_as_str = getSquareName(row, column);
        console.log(square_as_str);
        if (this.selectedSquare != null) {
            let should_return = this.selectedSquare == square_as_str;
            await this.UnselectSquare();
            if (should_return) {
                return;
            }
        }
        if (this.getPosition().board[row][column] == "") return;
        console.log('setting Square: ' + square_as_str);
        this.selectedSquare = square_as_str;
        let movesMap = await this.getPosition().movesMap;
        console.log(movesMap);
        console.log(square_as_str);
        console.log(movesMap.get(square_as_str));
        let moves = movesMap.get(square_as_str);
        if(typeof moves !== "undefined") {
            moves.forEach((sq) => {
                Singleton().drawCircle(sq);
            });
        }
    }

    async UnselectSquare() {
        if (this.selectedSquare == null) {
            console.log("Unexpectedly tried to unselected square when no square was selected!");
            return;
        }
        console.log('unselecting square');
        let moves = await (await this.getPosition().movesMap).get(this.selectedSquare);
        if (typeof moves !== "undefined") {

            await (moves.forEach((sq) => {
                let sqSvg = document.getElementById(sq);
                console.log('here');
                console.log(sqSvg.lastChild);
                sqSvg.childNodes.forEach((node) => {
                    console.log(node);
                    if (node.id == 'red_circle') {
                        sqSvg.removeChild(node);
                    }
                })
                console.log(sqSvg.lastChild);
            }));
        }
        this.selectedSquare = null;
    }

    undoMove() {
        if (this.boards.length === 0) {
            return;
        }
    }

    resetGame() {
        this.boards = [new Board()];
    }

    getPosition() {
        return this.boards[this.boards.length - 1];
    }

    gameLoop() {
        this.drawBoard();
        this.drawPieces();
    }

    drawCircle(toName) {
        let sqSvg = document.getElementById(toName);
        let clonedCircle = document.getElementById('red_circle').cloneNode(false);
        sqSvg.appendChild(clonedCircle);
    }


    maybedrawPieceAtSquare(i, j) {
        var svg = document.getElementById(getSquareName(i, j));
        var pos = this.getPosition();
        if (pos.board[i][j] == "") return;
        const sprite1 = document.getElementById(pos.board[i][j] + '1');
        const sprite2 = document.getElementById(pos.board[i][j] + '2');
        let auxNode = sprite1.cloneNode(false);
        svg.appendChild(auxNode);
        auxNode = sprite2.cloneNode(false);
        svg.appendChild(auxNode);
    }

    drawPieces() {
        for (var i = 0; i < 8; i++) {
            for (var j = 0; j < 8; j++) {
                if (this.flipped) {
                    this.maybedrawPieceAtSquare(7 - i, j);
                } else {
                    this.maybedrawPieceAtSquare(i, j);

                }
            }
        }
    }

    createSquare(i, j) {
        var square = document.createElement("div");
        square.setAttribute("width", 100);
        square.setAttribute("height", 100);
        var svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
        svg.setAttribute("width", "100");
        svg.setAttribute("height", "100");
        svg.setAttribute("id", getSquareName(i, j));
        square.appendChild(svg);
        square.className = "square " + ((i + j) % 2 === 0 ? "white" : "black");
        svg.addEventListener("click", svgclick);
        svg.dataset.row = i;
        svg.dataset.column = j;
        square.dataset.row = i;
        square.dataset.column = j;
        return square;
    }

    drawBoard() {
        var board_div = document.getElementById("chessboard");
        // Remove all rows before drawing.
        while (board_div.firstChild) {
            board_div.removeChild(board_div.firstChild);
        }
        let pos = this.getPosition();

        for (var i = 0; i < 8; i++) {
            var row = document.createElement("div");
            row.className = "row";
            for (var j = 0; j < 8; j++) {
                if (this.flipped) {
                    row.appendChild(this.createSquare(7 - i, j));
                } else {
                    row.appendChild(this.createSquare(i, j));

                }
            }
            board_div.appendChild(row);
        }
    }
}


const numToRow = {
    0: "A",
    1: "B",
    2: "C",
    3: "D",
    4: "E",
    5: "F",
    6: "G",
    7: "H"
};


function getSquareName(row, column) {
    const number = parseInt(row) + 1;
    const letter = numToRow[column];
    return `${letter}${number}`;
}

function convert_piece(piece_as_str) {
    if (piece_as_str == "black_rook" || piece_as_str == "white_rook") {
        return "R";
    } else if (piece_as_str == "black_bishop" || piece_as_str == "white_bishop") {
        return "B";
    } else if (piece_as_str == "black_knight" || piece_as_str == "white_knight") {
        return "N";
    } else if (piece_as_str == "black_queen" || piece_as_str == "white_queen") {
        return "Q";
    } else if (piece_as_str == "black_king" || piece_as_str == "white_king") {
        return "K";
    } else if (piece_as_str == "black_pawn" || piece_as_str == "white_pawn") {
        return "P";
    }
    return ".";
}

// Encodes a chessboard into a string.
function EncodeBoard(board) {
    let output = "";
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            output += this.convert_piece(board.board[i][j]);
        }
    }
    return output;
}

async function parsePossibleMoves(possibleMoves) {
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

async function svgclick(event) {
    let elem = event.target;
    // console.log(event);
    while (elem.tagName != 'DIV') {
        console.log('going up');
        console.log(elem.tagName);
        elem = elem.parentElement;
        console.log(elem);
    }
    console.log('svg clock');
    let row = elem.dataset.row;
    let column = elem.dataset.column;
    console.log(event);
    console.log(elem);
    console.log(elem.dataset);
    console.log(row);
    console.log(column);
    let movesMap = await Singleton().getPosition().movesMap;


    await Singleton().SelectSquare(row, column);
    event.stopPropagation();
}

async function flip_board(event) {
    Singleton().flip_board();
}

async function IssuePossibleMovesReq(board) {
    let uci_req = {
        board: EncodeBoard(board),
        req_type: "possible_moves",
        timeout: 3000,
    }
    let uci_response = await IssueUciReq(uci_req);
    console.log(uci_response);
    return uci_response.possible_moves;
}

async function IssueUciReq(uci_req) {
    const url = "http://127.0.0.1:9999";
    console.log('Sepnding request to engine..');
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

class UciRequest {
    constructor(board, req_type) {
        this.board = EncodeBoard(board);
        this.req_type = req_type;
        this.timeout = 3000;
    }
}

// class UciResponse {
//     constructor() { }
//     decode(response) {
//         this.best_moves = DecodeBestMoves(response);
//         // this.
//     }

//     // Gets the move from a move-eval string.
//     static GetMove(move_evals_as_str) {
//         let parts = move_evals_as_str.split(" ");
//         return parts[0];
//     }

//     // Gets the score from a move-eval string.
//     static GetScore(move_evals_as_str) {
//         let parts = move_evals_as_str.split(" ");
//         return parts[1];
//     } xx

//     static DecodeMoveEvals(move_evals_as_str) {
//         let Move = GetMove(move_evals_as_str);
//         let score = GetScore(move_evals_as_str);
//         return {
//             Move: Move,
//             Score: score
//         };
//     }

//     // Decodes a best moves string into an array of moves.
//     static DecodeBestMoves(response) {
//         let moves = [];
//         let aux = "";
//         for (let i = 0; i < response.length; i++) {
//             if (response[i] == " ") {
//                 moves.push(DecodeMoveEvals(aux));
//                 aux = "";
//             } else {
//                 aux += response[i];
//             }
//         }
//         moves.push(currentMove);
//         return moves;
//     }
// }

let controller = null;
function Singleton() {
    if (controller != null) {
        return controller;
    } else {
        controller = new Controller();
        return controller;
    }
}


var board = document.getElementById("chessboard");

// while (true) {
Singleton().gameLoop();
var button = document.getElementById('flip');
button.addEventListener("click", flip_board);
// }