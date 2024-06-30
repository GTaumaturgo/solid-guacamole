import { Board } from "./board.js"
import { IssuePossibleMovesReq, parsePossibleMoves, IssuePositionEvalReq } from "./client.js";
export class BoardStateManager {
  constructor(initialBoardState = new Board()) {
    this.boardState = []
    this.push_state(initialBoardState);
  }

  push_state(new_position) {
    new_position.setMoveMap(parsePossibleMoves(IssuePossibleMovesReq(new_position, new_position.to_move)));
    let pos_score = IssuePositionEvalReq(new_position, new_position.to_move);
    console.log('New position scoore:');
    console.log(pos_score);
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