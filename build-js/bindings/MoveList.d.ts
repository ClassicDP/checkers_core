import type { MoveItem } from "./MoveItem";
import type { Strike } from "./Strike";
export interface MoveList {
    list: Array<MoveItem>;
    current_chain: Strike;
}
