import type { Piece } from "./Piece";
import type { PosState } from "./PosState";
export interface Position {
    cells: Array<Piece | null>;
    state: PosState;
}
