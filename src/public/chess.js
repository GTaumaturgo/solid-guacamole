

function mousedown(event) {
    console.log("mousedown!");
    console.log(event.target.dataset.row);
    console.log(event.target.dataset.column);
}

function mouseup(event) {
    console.log("mouseup!");
    console.log(event.target.dataset.row);
    console.log(event.target.dataset.column);
    let board = controller.getPosition();
    console.log(board);
    let uci_req = new UciRequest(board,"best_move");
    callApi(uci_req);
}


async function callApi(uci_req) {
    const url = "http://127.0.0.1:8000/";
    let response = await fetch(url, {
        method: "POST",
        body: JSON.stringify(uci_req),
        headers: {
            "Content-Type": "application/json"
        }
    }).then(response => response.json())
    console.log(response);
}

class UciRequest {
    
    convert_piece(piece_as_str) {
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
    EncodeBoard(board) {
        let output = "";
        console.log("here");
        console.log(board);
        for (let i = 0; i < 8; i++) {
            for (let j = 0; j < 8; j++) {
                console.log(board.board[i][j]);
                output += this.convert_piece(board.board[i][j]);
            }
        }
        return output;
    }
    constructor(board, req_type) {
        this.board = this.EncodeBoard(board);
        this.req_type = req_type;
        this.timeout = 3000;
    }
}

class UciResponse {
    constructor() { }
    decode(response) {
        this.best_moves = DecodeBestMoves(response);
    }

    // Gets the move from a move-eval string.
    static GetMove(move_evals_as_str) {
        let parts = move_evals_as_str.split(" ");
        return parts[0];
    }

    // Gets the score from a move-eval string.
    static GetScore(move_evals_as_str) {
        let parts = move_evals_as_str.split(" ");
        return parts[1];
    }

    static DecodeMoveEvals(move_evals_as_str) {
        let Move = GetMove(move_evals_as_str);
        let score = GetScore(move_evals_as_str);
        return {
            Move: Move,
            Score: score
        };
    }

    // Decodes a best moves string into an array of moves.
    static DecodeBestMoves(response) {
        let moves = [];
        let aux = "";
        for (let i = 0; i < response.length; i++) {
            if (response[i] == " ") {
                moves.push(DecodeMoveEvals(aux));
                aux = "";
            } else {
                aux += response[i];
            }
        }
        moves.push(currentMove);
        return moves;
    }
}

class Board {
    constructor() {
        this.board = [
            ["black_rook", "black_knight", "black_bishop", "black_queen", "black_king", "black_bishop", "black_knight", "black_rook"],
            ["black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn", "black_pawn"],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn", "white_pawn"],
            ["white_rook", "white_knight", "white_bishop", "white_queen", "white_king", "white_bishop", "white_knight", "white_rook"]
        ];
    }

    setPiece(piece, row, col) {
        this.board[row][col] = piece;
    }

    getPiece(row, col) {
        return this.board[row][col];
    }

}

class Controller {

    constructor() {
        this.boards = [new Board()];
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
        console.log("get pos");
        return this.boards[this.boards.length - 1];
    }

    gameLoop() {
        this.drawPosition();
    }

    drawPosition() {
        var board_div = document.getElementById("chessboard");
        while (board_div.firstChild) {
            board_div.removeChild(board_div.firstChild);
        }
        let pos = this.getPosition();
        for (var i = 0; i < 8; i++) {
            var row = document.createElement("div");
            row.className = "row";
            for (var j = 0; j < 8; j++) {
                var square = document.createElement("div");
                square.className = "square " + ((i + j) % 2 === 0 ? "white" : "black");
                if (pos.board[i][j] !== "") {
                    const myElement = document.getElementById(pos.board[i][j]);
                    const clonedElement = myElement.cloneNode(true);
                    square.appendChild(clonedElement);
                }
                square.dataset.row = i;
                square.dataset.column = j;
                square.dataset.cont = this;
                square.addEventListener("mousedown", mousedown);
                square.addEventListener("mouseup", mouseup);
                row.appendChild(square);
            }
            board_div.appendChild(row);
        }
    }
}

var board = document.getElementById("chessboard");

const controller = new Controller();
// while (true) {
controller.gameLoop();
// }