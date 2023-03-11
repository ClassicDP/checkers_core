/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_straightstrike_free(a: number): void;
export function __wbg_positionhistoryitem_free(a: number): void;
export function __wbg_positionenvironment_free(a: number): void;
export function __wbg_get_positionenvironment_size(a: number): number;
export function __wbg_set_positionenvironment_size(a: number, b: number): void;
export function positionenvironment_new(a: number): number;
export function positionenvironment_js(a: number): number;
export function positionenvironment_is_king_move_for(a: number, b: number, c: number): number;
export function positionenvironment_game(): void;
export function positionenvironment_test(): number;
export function __wbg_moveitem_free(a: number): void;
export function __wbg_bestpos_free(a: number): void;
export function __wbg_game_free(a: number): void;
export function game_new(a: number): number;
export function game_set_depth(a: number, b: number): void;
export function game_insert_piece(a: number, b: number): void;
export function game_remove_piece(a: number, b: number): number;
export function game_position(a: number): number;
export function game_make_move_by_pos_item(a: number, b: number): void;
export function game_make_move_by_move_item(a: number, b: number): void;
export function game_best_move(a: number, b: number, c: number, d: number, e: number): number;
export function game_get_best_move(a: number): number;
export function game_make_best_move(a: number, b: number): void;
export function game_find_and_make_best_move_ts_n(a: number): number;
export function game_get_board_list_ts_n(a: number): number;
export function game_move_by_index_ts_n(a: number, b: number): number;
export function game_get_best_move_rust(a: number): number;
export function game_state_(a: number, b: number): void;
export function game_to_board(a: number, b: number): number;
export function game_to_pack(a: number, b: number): number;
export function game_get_move_list_for_front(a: number): number;
export function game_get_color(a: number): number;
export function game_set_color(a: number, b: number): void;
export function game_finish_check(a: number): number;
export function game_make_move_for_front(a: number, b: number, c: number): void;
export function piece_new(a: number, b: number, c: number): number;
export function piece_new_fom_js(a: number): number;
export function piece_it(a: number): number;
export function piece_set_it(a: number, b: number): void;
export function __wbg_piece_free(a: number): void;
export function __wbg_get_piece_pos(a: number): number;
export function __wbg_set_piece_pos(a: number, b: number): void;
export function __wbg_get_piece_color(a: number): number;
export function __wbg_set_piece_color(a: number, b: number): void;
export function __wbg_get_piece_is_king(a: number): number;
export function __wbg_set_piece_is_king(a: number, b: number): void;
export function __wbg_get_piece_stricken(a: number): number;
export function __wbg_set_piece_stricken(a: number, b: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
