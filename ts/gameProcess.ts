import * as wasm from "../pkg/checkers_core"
import {Color} from "../pkg";
import {Position} from "../bindings/Position";
import {ColorType} from "../bindings/ColorType";
import {MoveList} from "../bindings/MoveList";
import {MoveItem} from "../bindings/MoveItem";
import {Piece} from "../bindings/Piece";

export type BoardPos = number

type MoveChainElement = {
    from: BoardPos, to: BoardPos, take?: BoardPos,
    kingMove?: boolean,
}
type MoveVariants = {
    list?: MoveChainElement[]
    confirmed: MoveChainElement | undefined, done?: boolean
}

export class GameProcess {
    game: wasm.Game
    moveColor: Color
    private strikeChainInd: number = 0
    private moveList?: MoveList
    private moveChainPack: BoardPos[] = []

    static color(color?: ColorType): Color | undefined {
        if (!color) return undefined
        return color == "White" ? Color.White : Color.Black
    }

    constructor(size: number, color?: Color) {
        this.game = new wasm.Game(size)
        if (color) this.moveColor = color
    }

    invertMoveColor() {
        this.moveColor = this.moveColor == Color.Black ? Color.White : Color.Black
    }

    insertPiece(pos: number, color: Color, isKing: boolean) {
        this.game.insert_piece(wasm.Piece.new(this.game.to_pack(pos), color, isKing))
    }

    get position(): Position {
        let pos = this.game.position as Position
        let newPos: Position = {cells: []}
        for (let piece of pos.cells) {
            if (piece) newPos.cells[this.game.to_board(piece.pos)] = <Piece> {
                pos: this.game.to_board(piece.pos),
                color: piece.color,
                is_king: piece.is_king,
                stricken: piece.stricken
            }
        }
        return newPos
    }

    private frontClick(pos: BoardPos): MoveVariants {
        let getMoveChainElements = (moveList: MoveList | undefined, i: number) => {
            if (moveList?.list.length) {
                let moveKey: keyof MoveItem = moveList.list[0].strike ? 'strike' : 'mov'
                let res: MoveChainElement[] = []
                for (let move of moveList.list) {
                    if (moveKey == 'strike') {
                        let candidate = move[moveKey]!.vec[i]
                        if (candidate) res.push(
                            {
                                from: this.game.to_board(candidate.from),
                                to: this.game.to_board(candidate.to),
                                kingMove: candidate.king_move,
                                take: this.game.to_board(candidate.take)
                            })
                    } else {
                        res.push({
                            from: this.game.to_board(move[moveKey]!.from), to: this.game.to_board(move[moveKey]!.to),
                            kingMove: move[moveKey]!.king_move
                        })
                    }
                }
                return res
            }
            return []
        }


        let color = GameProcess.color((this.game.position as Position).cells[this.game.to_pack(pos)]?.color)
        if (!this.moveList) {
            if (!color) return {confirmed: undefined}
            this.moveList = <MoveList>this.game.get_move_list(color!)
        }
        let moveItems = getMoveChainElements(this.moveList, this.strikeChainInd)
        if (!moveItems.length) {
            if (this.strikeChainInd) {
                this.strikeChainInd = 0;
                return {done: true, confirmed: undefined}
            }
            return {confirmed: undefined}
        }
        let moveItems_ = moveItems.filter(x => x.to == pos)
        if (moveItems_.length) {
            let isStrike = moveItems_[0].take !== undefined
            if (isStrike) {
                this.moveList.list = this.moveList.list.filter(x =>
                    x.strike!.vec[this.strikeChainInd]?.to == this.game.to_pack(pos))
                let confirmed = this.moveList.list[0].strike!.vec[this.strikeChainInd++]
                let done = this.moveList.list.length == 1 &&
                    this.moveList.list[0].strike!.vec.length == this.strikeChainInd
                if (done) {
                    this.moveList = undefined
                    this.strikeChainInd = 0
                }
                return {
                    done: done,
                    list: done ? undefined : getMoveChainElements(this.moveList, this.strikeChainInd),
                    confirmed
                }
            } else {
                let confirmed = <MoveChainElement>this.moveList.list[0].mov
                this.moveList = undefined
                return {done: true, list: moveItems_, confirmed}
            }
        }
        // if user solve to change move piece
        if (!this.strikeChainInd) {
            let moveItems_ = moveItems.filter(x => x.from == pos)
            if (moveItems_.length) return {list: moveItems_, confirmed: undefined}
        }
        return {confirmed: undefined}
    }

    applyFrontClick(pos: number): MoveVariants {
        let variants = this.frontClick(pos)

        if (variants.confirmed) {
            variants.confirmed = <MoveChainElement>{
                from: this.game.to_board(variants.confirmed.from),
                to: this.game.to_board(variants.confirmed.to),
                take: !variants.confirmed.take ? false : this.game.to_board(variants.confirmed.take),
                kingMove: variants.confirmed.kingMove
            }
            if (!this.moveChainPack.length) {
                this.moveChainPack.push(variants.confirmed.from, variants.confirmed.to)
            } else {
                this.moveChainPack.push(variants.confirmed.to)
            }
        }
        if (variants.done) {
            this.game.make_move(this.moveChainPack.map(x => this.game.to_pack(x)))
            this.moveChainPack = []
        }

        return variants
    }
}



