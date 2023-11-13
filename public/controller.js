import { BoardStateManager } from "./board_state_manager.js"
import { IssuePossibleMovesReq } from "./client.js"
import { getSquareName, PIECE_DIV_SUFFIX, OVERLAY_DIV_SUFFIX } from "./common.js"


/** The Controller class is responsible for managing all the DOM*/
export class Controller {
    constructor() {
        this.piece_to_class = new Map([
            ["p", "white-pawn"],
            ["n", "white-knight"],
            ["b", "white-bishop"],
            ["r", "white-rook"],
            ["q", "white-queen"],
            ["k", "white-king"],
            ["P", "black-pawn"],
            ["N", "black-knight"],
            ["B", "black-bishop"],
            ["R", "black-rook"],
            ["Q", "black-queen"],
            ["K", "black-king"],
        ]);

        this.board_state_manager = new BoardStateManager();

        /** @type {?string}. The square that is currently selected.*/
        this.selectedSquare = null;

        /** @type {boolean}  Whether the board is flipped. */
        this.flipped = true;
        /** @type {Set<string>}. The set of squares that the current piece can move to. */
        this.canMoveTo = new Set();
    }

    async init() {
        let cur_pos = this.getCurPosition();
        let possible_moves_raw_response = IssuePossibleMovesReq(cur_pos);
        let move_map = this.parsePossibleMoves(possible_moves_raw_response);
        cur_pos.setMoveMap(move_map);
    }

    async parsePossibleMoves(possibleMoves) {
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

    /** Flips the `flipped` variable. */
    flip_board() {
        this.flipped = !this.flipped;
    }

    /** Gets the row index of the square with the given name. 
     * @param {string} sqName The name of the square.
     * @returns {number} The row index of the square. */
    GetRowFromSquareName(sqName) {
        return sqName[1].charCodeAt(0) - '1'.charCodeAt(0);
    }

    /**Gets the column index of the square with the given name.
     * @param {string} sqName The name of the square.
     * @returns {number} The column index of the square. */
    GetColumnFromSquareName(sqName) {
        return sqName[0].charCodeAt(0) - 'A'.charCodeAt(0);
    }

    async executeMove(fromSquare, toSquare) {
        /** Executes a move from the given fromSquare to the given toSquare.
         * @param {string} fromSquare The name of the square from which the piece is moving.
         * @param {string} toSquare The name of the square to which the piece is moving. */

        console.log('executing move');
        let pos = this.getCurPosition();
        let row_from = this.GetRowFromSquareName(fromSquare);
        let row_to = this.GetRowFromSquareName(toSquare);
        let col_from = this.GetColumnFromSquareName(fromSquare);
        let col_to = this.GetColumnFromSquareName(toSquare);
        console.log(row_from);
        console.log(col_from);
        console.log(row_to);
        console.log(col_to);

        let piece_as_str = pos.getPiece(row_from, col_from);
        this.undrawPieceAtSquare(row_from, col_from);
        pos.board.setPiece(row_from, col_from, ".");
        pos.board.setPiece(row_to, col_to, piece_as_str);

        this.maybeDrawPieceAtSquare(row_to, col_to);
        console.log(pos);
        pos.setMoveMap(parsePossibleMoves(IssuePossibleMovesReq(pos)));
        console.log(this.getCurPosition().movesMap)
        this.board_state_manager.push(Object.assign(this.getCurPosition()));
        console.log(this.board_state_manager);
    }

    async SelectSquare(row, column) {
        /**Selects the square at the given row and column.
         * @param {number} row The row index of the square to select.
         * @param {number} column The column index of the square to select. */
        let square_as_str = getSquareName(row, column);
        if (this.getCurPosition().getPiece(row, column) == "") return;
        this.selectedSquare = square_as_str;
        let movesMap = await this.getCurPosition().movesMap;
        let moves = movesMap.get(square_as_str);
        console.log(moves);
        if (typeof moves !== "undefined") {
            moves.forEach((sq) => {
                this.canMoveTo.add(sq);
                this.drawOverlay(sq);
            });
        }
    }

    /** Unselects the currently selected square. */
    async UnselectSquare() {
        if (this.selectedSquare == null) {
            console.log("Unexpectedly tried to unselected square when no square was selected!");
            return;
        }
        console.log("movemap");
        let movesMap = await this.getCurPosition().movesMap;
        console.log(movesMap);
        let moves = movesMap.get(this.selectedSquare);
        console.log(moves);
        console.log(this.selectedSquare);
        if (typeof moves !== "undefined") {
            console.log("unselect if");
            moves.forEach((sq) => {
                let sqElem = document.getElementById(sq + OVERLAY_DIV_SUFFIX);
                sqElem.classList.remove('red-overlay');
            });
        }
        this.canMoveTo.clear();
        this.selectedSquare = null;
    }

    undrawPieceAtSquare(i, j) {
        /** Removes the piece at the given row and column from the DOM. 
         * @param {number} i The row index of the square.
         * @param {number} j The column index of the square. */
        var piece_div = document.getElementById(getSquareName(i, j) + PIECE_DIV_SUFFIX);
        piece_div.setAttribute("class", "");
    }

    /** Undoes the last move that was made.*/
    undoMove() {
        if (this.board_state_manager.length === 0) {
            return;
        }
        this.board_state_manager.pop();
        this.selectedSquare = null;
        this.canMoveTo.clear();
    }

    resetGame() {
        /**Resets the game to the starting position. */
        this.board_state_manager = new BoardStateManager();
    }

    /** Returns the current game position. */
    getCurPosition() {
        return this.board_state_manager.peek();
    }

    gameLoop() {
        var board_div = document.getElementById("chessboard");
        this.removeAllChildElements(board_div);
        for (var i = 0; i < 8; i++) {
            for (var j = 0; j < 8; j++) {
                board_div.appendChild(this.createSquareNode(i, j));
                this.maybeDrawPieceAtSquare(i, j);
            }
        }
    }

    maybeDrawPieceAtSquare(i, j) {
        console.log("draw piece");
        /**Draws the piece at the given row and column to the DOM if the square is not empty.
         * @param {number} i The row index of the square.
         * @param {number} j The column index of the square. */
        if (this.flipped) {
            i = 7 - i;
            j = 7 - j;
        }
        var pos = this.getCurPosition();
        // Check if has piece.
        if (pos.getPiece(i, j) == "") return;
        var piece_div = document.getElementById(getSquareName(i, j) + PIECE_DIV_SUFFIX);
        console.log(getSquareName(i, j) + PIECE_DIV_SUFFIX);
        console.log(this.piece_to_class.get(pos.getPiece(i, j)));
        piece_div.setAttribute("class", this.piece_to_class.get(pos.getPiece(i, j)));
    }

    drawOverlay(toName) {
        /** Draws a red circle on the square Fwith the given name.
         * @param {string} toName The name of the square to draw the circle on. */
        console.log(toName);
        let sqElem = document.getElementById(toName + OVERLAY_DIV_SUFFIX);
        if (sqElem == undefined) {
            console.log("draw circle failed!!");
            return;
        }
        sqElem.classList.add("red-overlay");
    }

    removeAllChildElements(node_div) {
        while (node_div.firstChild) {
            node_div.removeChild(node_div.firstChild);
        }
    }

    createSquareNode(i, j) {
        /** Creates a square div with the given row and column indices.
        * @param {number} i The row index of the square.
        * @param {number} j The column index of the square.
        * @returns {Element} The created square div. */
        if (this.flipped) {
            i = 7 - i;
            j = 7 - j;
        }
        var piece_div = document.createElement("div");

        piece_div.style.height = '100px';
        piece_div.style.width = '100px';
        piece_div.setAttribute("id", getSquareName(i, j) + PIECE_DIV_SUFFIX);

        var square = document.createElement("div");
        square.style.gridArea = `${i + 1} / ${j + 1} / ${i + 2} / ${j + 2}`
        square.addEventListener("click", squareOnClick);
        square.setAttribute("class", "square " + ((i + j) % 2 === 0 ? "white" : "brown"));
        square.setAttribute("id", getSquareName(i, j));
        square.dataset.row = i;
        square.dataset.column = j;
        square.appendChild(piece_div);

        var overlay_div = document.createElement("div");
        overlay_div.setAttribute("id", getSquareName(i, j) + OVERLAY_DIV_SUFFIX);
        overlay_div.style.height = '100px';
        overlay_div.style.width = '100px';
        square.appendChild(overlay_div);
        return square;
    }
}

let internal_instance = null

export function ControllerInstance() {
    if (internal_instance != null) {
        return internal_instance;
    } else {
        internal_instance = new Controller();
        internal_instance.init();
        return internal_instance;
    }
}

// Callback functions for UI Elements
async function squareOnClick(event, debug = true) {
    console.log('squareOnClick');
    let elem = event.target;
    console.log(elem);
    // Climb up the DOM until we get to the right one.
    console.log('going up');
    console.log(elem.tagName);
    elem = elem.parentElement;
    console.log(elem);

    let row = elem.dataset.row;
    let column = elem.dataset.column;
    let movesMap = await (ControllerInstance().getCurPosition()).movesMap;
    let sq_as_str = getSquareName(row, column);
    let cur_selected = ControllerInstance().selectedSquare;
    if (debug) {
        console.log('currently qselected square:' + cur_selected);
        console.log('changing to:' + sq_as_str);
        console.log(ControllerInstance().canMoveTo);
    }

    if (cur_selected != null) {
        ControllerInstance().UnselectSquare();
        return;
    } else if (cur_selected == sq_as_str) {
        await ControllerInstance().UnselectSquare();
        return;
    } else if (cur_selected != null && ControllerInstance().canMoveTo.has(sq_as_str)) {
        console.log("unimplemented");
        ControllerInstance().executeMove(cur_selected, sq_as_str);
        ControllerInstance().UnselectSquare();
        return;
    }
    await ControllerInstance().SelectSquare(row, column);
    event.stopPropagation();
}


