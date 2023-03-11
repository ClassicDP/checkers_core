import type { PositionAndMove } from "./PositionAndMove";
export interface BestPos {
    pos: PositionAndMove | null;
    deep_eval: number;
}
