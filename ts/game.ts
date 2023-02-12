import * as wasm from "../pkg/lib1"
import {Color} from "../pkg";
import {Strike} from "../bindings/Strike";
import {Position} from "../bindings/Position";
import {ColorType} from "../bindings/ColorType";
import {MoveList} from "../bindings/MoveList";
import {MoveItem} from "../bindings/MoveItem";


type MoveIterationsItem = {
    from: number, to: number, take?: number,
    kingMove?: boolean,
}
type MoveIterations = {
    list?: MoveIterationsItem[]
    void?: boolean, done?: boolean
}

class Game {
    game: wasm.Game
    moveColor: Color
    private moveChainInd: number = 0
    private moveList?: MoveList

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


    frontClick(pos: number): MoveIterations {
        let getMoveIterationsItems = (moveList: MoveList | undefined, i: number ) => {
            if (moveList?.list.length) {
                let moveKey: keyof MoveItem = moveList.list[0].strike ? 'strike' : 'mov'
                let res: MoveIterationsItem[] = []
                for (let move of moveList.list) {
                    if (moveKey == 'strike') {
                        let candidate = move[moveKey]!.vec[i]
                        if (candidate) res.push(
                            {
                                from: candidate.from,
                                to: candidate.to,
                                kingMove: candidate.king_move,
                                take: candidate.take
                            })
                    } else {
                        res.push({
                            from: move[moveKey]!.from, to: move[moveKey]!.to, kingMove: move[moveKey]!.king_move
                        })
                    }
                }
                return res
            }
            return []
        }

        let color = Game.color((this.game.position as Position).cells[this.game.to_pack(pos)]?.color)
        if (!this.moveChainInd && color == this.moveColor) this.moveList = this.game.get_move_list(color)
        let moveItems = getMoveIterationsItems(this.moveList, this.moveChainInd)
        if (!moveItems.length) {
            if (this.moveChainInd) {
                this.moveChainInd = 0;
                return {done: true}
            }
            return {void: true}
        }
        if (!this.moveChainInd) {
            let moveItems_ = moveItems.filter(x => x.from == pos)
            if (moveItems_.length) return {list: moveItems_}
        }
        let moveItems_ = moveItems.filter(x => x.to == pos)
        if (moveItems_.length) {
            let isStrike = moveItems_[0].take !== undefined
            if (isStrike) {
                this.moveChainInd++
                return {list: moveItems_}
            } else {
                return {done: true}
            }
        }
        return {void: true}
    }

}



