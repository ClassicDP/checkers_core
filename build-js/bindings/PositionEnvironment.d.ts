import type { Grade } from "./Grade";
import type { Vector } from "./Vector";
export interface PositionEnvironment {
    size: number;
    king_row_black: number;
    king_row_white: number;
    vectors_map: Array<Array<Vector<number>>>;
    board_to_pack: Array<number>;
    pack_to_board: Array<number>;
    cell_grade: Array<Grade>;
}
