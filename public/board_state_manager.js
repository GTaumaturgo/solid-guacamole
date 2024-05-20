import { Board } from "./board.js"
import { IssuePossibleMovesReq, parsePossibleMoves } from "./client.js";
export class BoardStateManager {
  constructor(initialBoardState = new Board()) {
    this.boardState = []
    this.push_state(initialBoardState);
  }

  push_state(new_position) {
    new_position.setMoveMap(parsePossibleMoves(IssuePossibleMovesReq(new_position, new_position.to_move)));
    this.boardState.push(new_position);
  }

  pop_state() {
    return this.boardState.pop();
  }

  peek() {
    return this.boardState[this.boardState.length - 1];
  }

  isEmpty() {
    return this.boardState.length === 0;
  }
}