import * as wasm from "../pkg/lib1"
import {Figure} from "../bindings/Figure";

let game = new wasm.Game(8);
let ch = wasm.Piece.new_fom_js(<Figure> {pos:0,color: "Black", stricken: false, is_king: true})
console.log(ch)
ch.free()
ch = wasm.Piece.new_fom_js(<Figure>{pos: 2, is_king: true, stricken: false, color: "Black"})
console.log(game.js())
let a = JSON.parse(game.js())
console.log(a)
console.log(ch)

