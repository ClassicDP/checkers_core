/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Color {
  Black = 0,
  White = 1,
}
/**
*/
export enum DrawType {
  Draw1 = 0,
  Draw2 = 1,
  Draw3 = 2,
  Draw4 = 3,
  Draw5 = 4,
}
export type BoardPos = number;

export interface QuietMove {
    from: BoardPos;
    to: BoardPos;
    king_move: boolean;
}

/**
*/
export class Game {
  free(): void;
/**
* @param {number} size
*/
  constructor(size: number);
/**
* @param {Piece} piece
*/
  insert_piece(piece: Piece): void;
/**
* @param {number} pos
* @returns {boolean}
*/
  remove_piece(pos: number): boolean;
/**
* @param {number} pack_index
* @returns {number}
*/
  to_board(pack_index: number): number;
/**
* @param {number} board_index
* @returns {number}
*/
  to_pack(board_index: number): number;
/**
* @returns {any}
*/
  get_move_list_for_front(): any;
/**
* @returns {number | undefined}
*/
  draw_check(): number | undefined;
/**
* @param {any} pos_chain
* @returns {any}
*/
  make_move_for_front(pos_chain: any): any;
/**
*/
  moveColor: number;
/**
*/
  readonly position: any;
/**
*/
  readonly state: any;
}
/**
*/
export class GameState {
  free(): void;
}
/**
*/
export class Piece {
  free(): void;
/**
* @param {number} pos
* @param {number} color
* @param {boolean} is_king
* @returns {Piece}
*/
  static new(pos: number, color: number, is_king: boolean): Piece;
/**
* @param {any} js
* @returns {Piece | undefined}
*/
  static new_fom_js(js: any): Piece | undefined;
/**
*/
  color: number;
/**
*/
  is_king: boolean;
/**
*/
  it: any;
/**
*/
  pos: number;
/**
*/
  stricken: boolean;
}
/**
*/
export class PositionEnvironment {
  free(): void;
/**
* @param {number} size
*/
  constructor(size: number);
/**
* @returns {any}
*/
  js(): any;
/**
* @param {Piece} piece
* @param {number} pos
* @returns {boolean}
*/
  is_king_move_for(piece: Piece, pos: number): boolean;
/**
*/
  static game(): void;
/**
* @returns {any}
*/
  static test(): any;
/**
*/
  size: number;
}
/**
*/
export class StraightStrike {
  free(): void;
}
