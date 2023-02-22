import { Color } from "../build-wasm/checkers_core";
import { Position } from "./bindings/Position";
import { ColorType } from "./bindings/ColorType";
import { MoveList } from "./bindings/MoveList";
export type BoardPos = number;
type MoveChainElement = {
    from: BoardPos;
    to: BoardPos;
    take?: BoardPos;
    kingMove?: boolean;
};
export type MoveVariants = {
    list?: MoveChainElement[];
    confirmed: MoveChainElement | undefined;
    done?: boolean;
};
export declare class GameProcess {
    private game;
    moveColor: Color;
    private strikeChainInd;
    private moveList?;
    private moveChainPack;
    static color(color?: ColorType): Color | undefined;
    constructor(size: number, color?: Color);
    invertMoveColor(): void;
    insertPiece(pos: number, color: Color, isKing: boolean): void;
    removePiece(pos: number): boolean;
    get position(): Position;
    private frontClick;
    getMoveList(color: Color): MoveList;
    applyFrontClick(pos: number): MoveVariants;
}
export {};
