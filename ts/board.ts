import * as wasm from "../pkg/lib1"
let game = new wasm.Game(8);
let ch = new wasm.Figure(0, 0, true)
ch.it = {pos: 1, color: "Black", is_king: false, stricken: false}
console.log(ch.it)
console.log(game.last_position)

