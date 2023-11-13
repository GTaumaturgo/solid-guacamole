import {Board} from "./board.js"

export class BoardStateManager {
    constructor(initialBoardState = new Board()) {
        this.boardState = [initialBoardState]
    }

    push(newState) {
        this.boardState.push(newState);
    }

    pop() {
        return this.boardState.pop();
    }

    peek() {
        return this.boardState[this.boardState.length - 1];
    }

    isEmpty() {
        return this.boardState.length === 0;
    }

    // Returns a copy of the current board state.
    copy() {
        return [...this.boardState];
    }
}