import {GameProcess, MoveVariants} from "./ts/gameProcess";
import {Color} from "./pkg";
import {Position} from "./bindings/Position";
import {GameState} from "./bindings/GameState";
import {MoveList} from "./bindings/MoveList";


// https://docs.google.com/document/d/1xliHnMDi1OAsQqN-aNkdamqCDXfD7RJT01xuvVTa_-o/edit#bookmark=id.1f5pzrhvjswx
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

        console.log(gameProcess.position)
        let move: MoveVariants
        console.log(move = gameProcess.applyFrontClick(47))
        expect(move.list[0].kingMove).toEqual(true)
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
        let state = gameProcess.position.state
        expect(state.black.king).toEqual(1)
        expect(state.white.king).toEqual(1)
        console.log(state)
        gameProcess.removePiece(54)
        state = gameProcess.position.state
        expect(state.black.king).toEqual(1)
        expect(state.white.king).toEqual(0)
        console.log(gameProcess.position.state)
        console.log(gameProcess.position.state)
    })

    // https://docs.google.com/document/d/1xliHnMDi1OAsQqN-aNkdamqCDXfD7RJT01xuvVTa_-o/edit#bookmark=id.oehnm5eas6gm
    test("move variants Strike", () => {
        let gameProcess = new GameProcess(8);
        gameProcess.insertPiece(0, Color.White, true);
        [9, 11, 13, 25, 27, 29, 41, 43, 45].forEach(i => gameProcess.insertPiece(i, Color.Black, false))
        let list = gameProcess.getMoveList(Color.White) as MoveList;
        console.log(list.list.map(x => x.strike.vec))
        expect(list.list.length).toEqual(42)
    })

    test("move variants Quite move", () => {
        let gameProcess = new GameProcess(8)
        gameProcess.insertPiece(27, Color.White, true);
        [4, 48, 54].forEach(i => gameProcess.insertPiece(i, Color.White, false))
        console.log(gameProcess.position)
        let list = gameProcess.getMoveList(Color.White);
        console.log(list.list.map(x => x.mov))
        expect(list.list.length).toEqual(15)
    })
});