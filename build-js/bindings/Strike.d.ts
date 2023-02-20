import type { Piece } from "./Piece";
import type { StraightStrike } from "./StraightStrike";
export interface Strike {
    vec: Array<StraightStrike>;
    took_pieces: Array<Piece | null>;
    king_move: boolean;
}
