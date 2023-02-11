import * as wasm from "../pkg/lib1"
import {Color} from "../pkg/lib1";



let game = new wasm.Game(8);
let ch = wasm.Piece.new(0, Color.Black, true);
game.insert_piece(ch);
console.log(game.position)



