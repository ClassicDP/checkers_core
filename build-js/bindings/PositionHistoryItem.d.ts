import type { MoveItem } from "./MoveItem";
import type { Position } from "./Position";
export interface PositionHistoryItem {
    position: Position;
    move_item: MoveItem;
}
