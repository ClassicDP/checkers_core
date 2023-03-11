import type { MoveItem } from "./MoveItem";
import type { Position } from "./Position";
export interface PositionAndMove {
    pos: Position;
    mov: MoveItem | null;
}
