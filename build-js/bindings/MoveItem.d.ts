import type { QuietMove } from "./QuietMove";
import type { Strike } from "./Strike";
export interface MoveItem {
    strike: Strike | null;
    mov: QuietMove | null;
}
