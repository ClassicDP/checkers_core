"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const wasm = __importStar(require("../pkg/checkers_core"));
const pkg_1 = require("../pkg");
class Game {
    constructor(size, color) {
        this.game = new wasm.Game(size);
        if (color)
            this.moveColor = color;
    }
    static color(color) {
        if (!color)
            return undefined;
        return color == "White" ? pkg_1.Color.White : pkg_1.Color.Black;
    }
    invertMoveColor() {
        this.moveColor = this.moveColor == pkg_1.Color.Black ? pkg_1.Color.White : pkg_1.Color.Black;
    }
    insertPiece(pos, color, isKing) {
        this.game.insert_piece(wasm.Piece.new(this.game.to_pack(pos), color, isKing));
    }
    frontClick(pos) {
        let color = Game.color(this.game.position.cells[this.game.to_pack(pos)]?.color);
        if (color == this.moveColor) {
            if (!this.strikeChainInd)
                this.moveList = this.game.get_move_list(color);
            if (this.moveList.list.length) {
                if (this.moveList.list[0].strike) {
                    if (this.strikeChainInd == undefined) {
                        this.strikeChainInd = 0;
                    }
                    else {
                        this.strikeChainInd++;
                    }
                    let moveIterationList = { lastChainItem: true };
                    moveIterationList.list = this.moveList.list
                        .filter(it => {
                        let strike = it.strike.vec[this.strikeChainInd];
                        if (strike !== undefined)
                            moveIterationList.lastChainItem = false;
                        return strike?.from == pos;
                    })
                        .map(it => {
                        let strike = it.strike.vec[this.strikeChainInd];
                        return {
                            from: strike.from,
                            to: strike.to,
                            take: strike.take,
                            kingMove: strike.king_move
                        };
                    });
                    if (this.moveList.list.length) {
                        this.strikeChainInd++;
                        return moveIterationList;
                    }
                    else {
                    }
                }
            }
        }
        return { void: true };
    }
}
//# sourceMappingURL=game.js.map