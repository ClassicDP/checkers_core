import type { PositionHistoryItem } from "./PositionHistoryItem";
export interface BestPos {
    pos: PositionHistoryItem | null;
    deep_eval: number;
}
