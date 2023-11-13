export class Board {
    #kInitialBoardState = [
        ["r", "n", "b", "q", "k", "b", "n", "r"],
        ["p", "p", "p", "p", "p", "p", "p", "p"],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["P", "P", "P", "P", "P", "P", "P", "P"],
        ["R", "N", "B", "Q", "K", "B", "N", "R"]];


    constructor(initial_state = this.#kInitialBoardState) {
        this.internal_state = initial_state
        this.movesMap = null
    }

    setPiece(row, col, piece) {
        this.internal_state[row][col] = piece;
    }

    getPiece(row, col) {
        console.log(row);
        console.log(col);
        console.log(this.internal_state[row][col]);
        return this.internal_state[row][col];
    }

    setMoveMap(movesMap) {
        this.movesMap = movesMap;
    }

    hasMovesMap() {
        return this.movesMap != null;
    }
}