/* tslint:disable */
/* eslint-disable */
/**
*/
export function test_q(): void;
/**
*/
export enum DrawType {
  draw1 = 0,
  draw2 = 1,
  draw3 = 2,
  draw4 = 3,
  draw5 = 4,
}
/**
*/
export enum Color {
  Black = 0,
  White = 1,
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
* @param {number} color
* @returns {any}
*/
  get_move_list_for_front(color: number): any;
/**
* @returns {number | undefined}
*/
  draw_check(): number | undefined;
/**
* @param {any} pos_chain
* @returns {boolean}
*/
  make_move_for_front(pos_chain: any): boolean;
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
export class ListP {
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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_positionenvironment_free: (a: number) => void;
  readonly __wbg_get_positionenvironment_size: (a: number) => number;
  readonly __wbg_set_positionenvironment_size: (a: number, b: number) => void;
  readonly positionenvironment_new: (a: number) => number;
  readonly positionenvironment_js: (a: number) => number;
  readonly positionenvironment_is_king_move_for: (a: number, b: number, c: number) => number;
  readonly positionenvironment_game: () => void;
  readonly positionenvironment_test: () => number;
  readonly __wbg_straightstrike_free: (a: number) => void;
  readonly piece_new: (a: number, b: number, c: number) => number;
  readonly piece_new_fom_js: (a: number) => number;
  readonly piece_it: (a: number) => number;
  readonly piece_set_it: (a: number, b: number) => void;
  readonly __wbg_piece_free: (a: number) => void;
  readonly __wbg_get_piece_pos: (a: number) => number;
  readonly __wbg_set_piece_pos: (a: number, b: number) => void;
  readonly __wbg_get_piece_color: (a: number) => number;
  readonly __wbg_set_piece_color: (a: number, b: number) => void;
  readonly __wbg_get_piece_is_king: (a: number) => number;
  readonly __wbg_set_piece_is_king: (a: number, b: number) => void;
  readonly __wbg_get_piece_stricken: (a: number) => number;
  readonly __wbg_set_piece_stricken: (a: number, b: number) => void;
  readonly __wbg_listp_free: (a: number) => void;
  readonly test_q: () => void;
  readonly __wbg_gamestate_free: (a: number) => void;
  readonly __wbg_game_free: (a: number) => void;
  readonly game_new: (a: number) => number;
  readonly game_insert_piece: (a: number, b: number) => void;
  readonly game_remove_piece: (a: number, b: number) => number;
  readonly game_position: (a: number) => number;
  readonly game_state: (a: number) => number;
  readonly game_to_board: (a: number, b: number) => number;
  readonly game_to_pack: (a: number, b: number) => number;
  readonly game_get_move_list_for_front: (a: number, b: number) => number;
  readonly game_draw_check: (a: number) => number;
  readonly game_make_move_for_front: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
