import { BoardStateManager } from "./board_state_manager.js"
import { GetLongClastingSquareforKing, IssuePossibleMovesReq, parsePossibleMoves } from "./client.js"
import { getSquareName, PIECE_DIV_SUFFIX, OVERLAY_DIV_SUFFIX, BLACK_PLAYER, WHITE_PLAYER } from "./common.js"

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
        this.flipped = false;
        /** @type {Set<string>}. The set of squares that the current piece can move to. */
        this.canMoveTo = new Set();
        this.to_move = WHITE_PLAYER;
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

    MoveSinglePieceAndUpdateInternalState(fromSquare, toSquare, new_position) {

        let row_from = this.GetRowFromSquareName(fromSquare);
        let row_to = this.GetRowFromSquareName(toSquare);
        let col_from = this.GetColumnFromSquareName(fromSquare);
        let col_to = this.GetColumnFromSquareName(toSquare);
        const cur_position = this.getCurPosition();
        let piece_as_str = cur_position.getPiece(row_from, col_from);


        this.undrawPieceAtSquare(row_from, col_from);
        // If there is other piece, undraw it (e.g.) a capture.
        if (cur_position.hasPieceAt(row_to, col_to)) {
            this.undrawPieceAtSquare(row_to, col_to);
        }
        new_position.setPieceAsEmpty(row_from, col_from);
        new_position.setPiece(row_to, col_to, piece_as_str);
    }
    async ExecuteShortCastlingForWhite() {
        this.ExecuteCastlingMove('E1', 'G1', 'H1', 'F1');
    }

    async ExecuteShortCastlingForBlack() {
        this.ExecuteCastlingMove('E8', 'G8', 'H8', 'F8');
    }

    async ExecuteLongCastlingForWhite() {
        this.ExecuteCastlingMove('E1', 'C1', 'A1', 'D1');
    }

    async ExecuteLongCastlingForBlack() {
        this.ExecuteCastlingMove('E8', 'C8', 'A8', 'D8');
    }

    async ExecuteCastlingMove(KingFromSquare, KingToSquare, RookFromSquare, RookToSquare) {
        let new_position = this.getCurPosition().copy_as_continuation();
        this.MoveSinglePieceAndUpdateInternalState(KingFromSquare, KingToSquare, new_position);
        this.MoveSinglePieceAndUpdateInternalState(RookFromSquare, RookToSquare, new_position);
        this.board_state_manager.push_state(new_position);
        // We need to draw after pushing the new position as it checks the position at the top of the stack.
        this.drawPieceAtSquareIfPresent(this.GetRowFromSquareName(KingToSquare), this.GetColumnFromSquareName(KingToSquare));
        this.drawPieceAtSquareIfPresent(this.GetRowFromSquareName(RookToSquare), this.GetColumnFromSquareName(RookToSquare))
        this.flip_player_to_move();
    }



    async executeMoveFull(fromSquare, toSquare, debug = true) {
        /** Executes a move from the given fromSquare to the given toSquare.
         * @param {string} fromSquare The name of the square from which the piece is moving.
         * @param {string} toSquare The name of the square to which the piece is moving. */
        let new_position = this.getCurPosition().copy_as_continuation();
        let row_to = this.GetRowFromSquareName(toSquare);
        let col_to = this.GetColumnFromSquareName(toSquare);

        this.MoveSinglePieceAndUpdateInternalState(fromSquare, toSquare, new_position);
        this.board_state_manager.push_state(new_position);

        // We need to draw after pushing the new position as it checks the position at the top of the stack.
        this.drawPieceAtSquareIfPresent(row_to, col_to);
        this.flip_player_to_move();
    }



    flip_player_to_move() {
        if (this.to_move == WHITE_PLAYER) {
            this.to_move = BLACK_PLAYER;
        } else {
            this.to_move = WHITE_PLAYER;
        }
    }

    async SelectSquare(row, column) {
        /**Selects the square at the given row and column.
         * @param {number} row The row index of the square to select.
         * @param {number} column The column index of the square to select. */
        if (!this.getCurPosition().hasPieceAt(row, column)) return;

        let square_as_str = getSquareName(row, column);
        this.selectedSquare = square_as_str;
        let movesMap = await this.getCurPosition().movesMap();
        // console.log(this.getCurPosition());
        // console.log(movesMap);
        // console.log(moves);
        let moves = movesMap.get(square_as_str);
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
        let movesMap = await this.getCurPosition().movesMap();
        console.log(await this.getCurPosition());
        console.log(movesMap);
        let moves = movesMap.get(this.selectedSquare);
        console.log(moves);
        console.log(this.selectedSquare);
        if (typeof moves !== "undefined") {
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
        piece_div.setAttribute("class", "piece-container");
    }

    /** Undoes the last move that was made.*/
    undoMove() {
        if (this.board_state_manager.length === 0) {
            return;
        }
        this.board_state_manager.pop_state();
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
        console.log('game loop');
        var board_div = document.getElementById("chessboard");
        this.removeAllChildElements(board_div);
        for (var i = 0; i < 8; i++) {
            for (var j = 0; j < 8; j++) {
                board_div.appendChild(this.createSquareNode(i, j));
                this.drawPieceAtSquareIfPresent(i, j);
            }
        }
    }

    drawPieceAtSquareIfPresent(i, j) {
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
        console.log(pos);
        console.log(i);
        console.log(j);
        console.log(pos.getPiece(i, j));
        if (!pos.hasPieceAt(i, j)) return;
        console.log('drawing');
        var piece_div = document.getElementById(getSquareName(i, j) + PIECE_DIV_SUFFIX);
        piece_div.classList.add(this.piece_to_class.get(pos.getPiece(i, j)));
    }

    drawOverlay(toName) {
        /** Draws a red circle on the square with the given name.
         * @param {string} toName The name of the square to draw the circle on. */
        console.log(toName);
        let sqElem = document.getElementById(toName + OVERLAY_DIV_SUFFIX);
        if (sqElem == undefined) {
            console.log("draw overlay failed!!");
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

        piece_div.setAttribute("id", getSquareName(i, j) + PIECE_DIV_SUFFIX);
        piece_div.classList.add("piece-container");
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
        return internal_instance;
    }
}

// Callback functions for UI Elements
async function squareOnClick(event, debug = true) {
    console.log('squareOnClick');
    let elem = event.target;

    elem = elem.parentElement;


    let row = elem.dataset.row;
    let column = elem.dataset.column;
    let movesMap = await (ControllerInstance().getCurPosition()).movesMap();
    let sq_as_str = getSquareName(row, column);
    let cur_selected = ControllerInstance().selectedSquare;
    if (debug) {
        console.log('currently selected square:' + cur_selected);
        console.log('clicked:' + sq_as_str);
        console.log(ControllerInstance().canMoveTo);
    }

    // Because the `canMoveTo` vector already has only the pieces of the side to move, we don't have to check that.
    if (cur_selected != null && ControllerInstance().canMoveTo.has(sq_as_str)) {
        ControllerInstance().UnselectSquare();
        // TODO ALSO CHECK ITS THE KING

        if (cur_selected == 'E1' && sq_as_str == 'G1') {
            ControllerInstance().ExecuteShortCastlingForWhite();
        } else if (cur_selected == 'E8' && sq_as_str == 'G8') {
            ControllerInstance().ExecuteShortCastlingForBlack();
        } else if (cur_selected == 'E1' && sq_as_str == 'C1') {
            ControllerInstance().ExecuteLongCastlingForWhite();
        } else if (cur_selected == 'E8' && sq_as_str == 'C8') {
            ControllerInstance().ExecuteLongCastlingForBlack();
        }
        else {
            ControllerInstance().executeMoveFull(cur_selected, sq_as_str);
        }

        return;
    } else if (cur_selected == sq_as_str) {
        await ControllerInstance().UnselectSquare();
        return;
    } else if (cur_selected != null) {
        ControllerInstance().UnselectSquare();
        return;
    }
    await ControllerInstance().SelectSquare(row, column);
    event.stopPropagation();
}


