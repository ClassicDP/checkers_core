import type { ColorType } from "./ColorType";
import type { MoveList } from "./MoveList";
import type { Piece } from "./Piece";
import type { PosState } from "./PosState";
export interface Position {
    cells: Array<Piece | null>;
    state: PosState;
    next_move: ColorType | null;
    move_list: MoveList | null;
}
