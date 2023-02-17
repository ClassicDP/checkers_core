import {GameProcess} from "./ts/gameProcess";
import {Color} from "./pkg";
import {Position} from "./bindings/Position";
import {GameState} from "./bindings/GameState";


describe("Game tests", () => {
    test("applyFrontClick", () => {
        let gameProcess = new GameProcess(8, Color.White);
        gameProcess.insertPiece(0, Color.White, true)
        gameProcess.insertPiece(63, Color.White, true)
        gameProcess.insertPiece(18, Color.Black, false)
        gameProcess.insertPiece(43, Color.Black, false)
        console.log(gameProcess.position)
        console.log(gameProcess.applyFrontClick(0))
        console.log(gameProcess.applyFrontClick(63))
        console.log(gameProcess.applyFrontClick(0))
        console.log(gameProcess.applyFrontClick(36))
        console.log(gameProcess.applyFrontClick(57))
        let pos = gameProcess.position as Position
        expect(pos.cells.filter(x => x !== undefined).length).toEqual(2)
        console.log(pos)
    });

    test("king move applyFrontClick", () => {
        let gameProcess = new GameProcess(8, Color.White);
        gameProcess.insertPiece(47, Color.White, false)
        gameProcess.insertPiece(54, Color.Black, true)
        gameProcess.insertPiece(52, Color.Black, false)
        gameProcess.insertPiece(29, Color.Black, false)
        gameProcess.insertPiece(13, Color.Black, false)
        gameProcess.insertPiece(18, Color.Black, false)
        gameProcess.insertPiece(34, Color.Black, false)
        gameProcess.insertPiece(50, Color.Black, false)

        console.log(gameProcess.game.position)
        console.log(gameProcess.applyFrontClick(47))
        console.log(gameProcess.applyFrontClick(61))
        console.log(gameProcess.applyFrontClick(43))
        console.log(gameProcess.applyFrontClick(22))
        console.log(gameProcess.applyFrontClick(4))
        console.log(gameProcess.applyFrontClick(25))
        console.log(gameProcess.applyFrontClick(43))
        console.log(gameProcess.applyFrontClick(57))
        let pos = gameProcess.position as Position
        expect(pos.cells.filter(x => x !== undefined).length).toEqual(1)
        console.log(pos)
    });

    test("insert and delete pieces", () => {
        let gameProcess = new GameProcess(8);
        gameProcess.insertPiece(54, Color.White, true)
        gameProcess.insertPiece(9, Color.Black, true)
        let state = (gameProcess.game.position as Position).state
        expect(state.black.king).toEqual(1)
        expect(state.white.king).toEqual(1)
        console.log(state)
        gameProcess.removePiece(54)
        state = (gameProcess.game.position as Position).state
        expect(state.black.king).toEqual(1)
        expect(state.white.king).toEqual(0)
        console.log((gameProcess.game.position as Position).state)
        console.log(gameProcess.game.state as GameState)
    })
});