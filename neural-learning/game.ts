import {Color, Game} from "../build-wasm";
import {GameProcess} from "../src/gameProcess";

do {
    let gameProcess = new GameProcess(8);
    let white_l = [0, 2, 4, 6, 9, 11, 13, 15, 16, 18, 20, 22]
    let black_l = white_l.map(x => 63 - x)
    white_l.forEach(x => gameProcess.insertPiece(x, Color.White, false))
    black_l.forEach(x => gameProcess.insertPiece(x, Color.Black, false))
    gameProcess.moveColor = Color.White;
    let list: [] | string
    do {
        list = gameProcess.game.find_and_make_best_move_ts_n()
        // gameProcess.game.move_by_index_ts_n(Math.random() * list.length >> 0)
        // if (!(list instanceof Array)) console.log(list)
        console.log(list)
    } while (list instanceof Array)

} while (1)


