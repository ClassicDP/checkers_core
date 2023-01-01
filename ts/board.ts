import * as wasm from "../pkg/lib1"
let game = wasm.Game.new(8);
let ch = new wasm.Figure(0, 0, true)
console.log(ch.it)
console.log(game.last_position)

