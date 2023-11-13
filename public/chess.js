import {Board} from "./board.js"
import {Controller, ControllerInstance} from "./controller.js"
import {numToRow, getSquareName} from "./common.js"

// Draw the Board and wait for move.
ControllerInstance().gameLoop();