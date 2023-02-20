import type { ColorType } from "./ColorType";
export interface Piece {
    pos: number;
    color: ColorType;
    is_king: boolean;
    stricken: boolean;
}
