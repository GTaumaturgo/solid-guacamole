import { WHITE_PLAYER } from "./common.js";

export class Board {
    #kEmpty = "."
    #kInitialBoardState = [
        ["r", "n", "b", "q", "k", "b", "n", "r"],
        ["p", "p", "p", "p", "p", "p", "p", "p"],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        ["P", "P", "P", "P", "P", "P", "P", "P"],
        ["R", "N", "B", "Q", "K", "B", "N", "R"]];


    constructor(initial_state = this.#kInitialBoardState, moves_map = null, to_move = WHITE_PLAYER) {
        this.internal_state = initial_state
        this.moves_map = moves_map
        this.to_move = to_move
    }

    setPiece(row, col, piece) {
        this.internal_state[row][col] = piece;
    }

    setPieceAsEmpty(row, col, piece) {
        this.internal_state[row][col] = this.#kEmpty;
    }

    getPiece(row, col) {
        return this.internal_state[row][col];
    }

    hasPieceAt(row, col) {
        return this.getPiece(row, col) != this.#kEmpty;
    }

    setMoveMap(moves_map) {
        this.moves_map = moves_map;
    }

    movesMap() {
        return this.moves_map;
    }

    hasMovesMap() {
        return this.movesMap != null;
    }

    copy() {
        return new Board(this.internal_state, this.moves_map, this.to_move);
    }
}