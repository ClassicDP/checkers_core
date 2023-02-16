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
    test("applyFrontClick", () => {
        let game = new Game(8, Color.White);
        game.insertPiece(0, Color.White, true)
        game.insertPiece(63, Color.White, true)
        game.insertPiece(18, Color.Black, false)
        game.insertPiece(43, Color.Black, false)
        console.log(game.game.position)
        console.log(game.applyFrontClick(0))
        console.log(game.applyFrontClick(63))
        console.log(game.applyFrontClick(0))
        console.log(game.applyFrontClick(36))
        console.log(game.applyFrontClick(57))
        let pos = game.game.position as Position
        expect(pos.cells.filter(x=>x!==undefined).length).toEqual(2)
        console.log(pos)
    });

    test("king move applyFrontClick", () => {
        let game = new Game(8, Color.White);
        game.insertPiece(47, Color.White, false)
        game.insertPiece(54, Color.Black, true)
        game.insertPiece(52, Color.Black, false)
        game.insertPiece(29, Color.Black, false)
        game.insertPiece(13, Color.Black, false)
        game.insertPiece(18, Color.Black, false)
        game.insertPiece(34, Color.Black, false)
        game.insertPiece(50, Color.Black, false)

        console.log(game.game.position)
        console.log(game.applyFrontClick(47))
        console.log(game.applyFrontClick(61))
        console.log(game.applyFrontClick(43))
        console.log(game.applyFrontClick(22))
        console.log(game.applyFrontClick(4))
        console.log(game.applyFrontClick(25))
        console.log(game.applyFrontClick(43))
        console.log(game.applyFrontClick(57))
        let pos = game.game.position as Position
        expect(pos.cells.filter(x=>x!==undefined).length).toEqual(1)
        console.log(pos)
    });
});