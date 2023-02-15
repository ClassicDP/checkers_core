import {Game} from "./ts/game";
import {Color} from "./pkg";
import {Position} from "./bindings/Position";

describe("This is a simple test", () => {
    test("Check the sampleFunction function", () => {
        let s = (a: number, b: number)=>a+b
        expect(s(1,2)).toEqual(3);
    });
});

describe("Game tests", () => {
    test("get move", () => {
        let game = new Game(8, Color.White);
        game.insertPiece(0, Color.White, true)
        game.insertPiece(9, Color.Black, false)
        game.insertPiece(43, Color.Black, false)
        console.log(game.applyMove(0))
        console.log(game.applyMove(36))
        console.log(game.applyMove(57))
        let pos = game.game.position as Position
        expect(pos.cells.filter(x=>x!==undefined).length).toEqual(1)
        console.log(pos)
    });
});