function mousedown(event) {
    console.log("mousedown!");
    console.log(event.target.dataset.row);
    console.log(event.target.dataset.column);
}

function mouseup(event) {
    console.log("mouseup!");
    console.log(event.target.dataset.row);
    console.log(event.target.dataset.column);
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
        // this.gameEngine = gameEngine;
        this.boards = [new Board()];
    }

    makeMove(from, to) {
        this.boards.push(this.gameEngine.getBoard());
        // this.gameEngine.makeMove(from, to);
    }

    undoMove() {
        if (this.boards.length === 0) {
            return;
        }
        // this.gameEngine.setBoard(this.boards.pop());
    }

    resetGame() {
        this.boards = [new Board()];
        // this.gameEngine.resetGame();
    }

    getPosition() {
        return this.boards[this.boards.length - 1];
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
                row.appendChild(square);
                if (pos.board[i][j] !== "") {
                    const myElement = document.getElementById(pos.board[i][j]);
                    const clonedElement = myElement.cloneNode(true);
                    square.appendChild(clonedElement);
                }
                square.dataset.row = i;
                square.dataset.column = j;
                // square.dataset.set("row", i)
                // square.dataset.set("column", j)
                square.addEventListener("mousedown", mousedown);
                square.addEventListener("mouseup", mouseup);
            }
            board_div.appendChild(row);
        }
    }
}



var board = document.getElementById("chessboard");
const controller = new Controller();
while (true) {
    controller.drawPosition();
    break;
}