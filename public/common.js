export const PIECE_DIV_SUFFIX = "_piece"
export const OVERLAY_DIV_SUFFIX = "_overlay";
export const WHITE_PLAYER = "W";
export const BLACK_PLAYER = "B";

export const numToRow = {
    0: "A",
    1: "B",
    2: "C",
    3: "D",
    4: "E",
    5: "F",
    6: "G",
    7: "H"
};

export function getSquareName(row, column) {
    const number = parseInt(row) + 1;
    const letter = numToRow[column];
    return `${letter}${number}`;
}

// export enum Color {

// }