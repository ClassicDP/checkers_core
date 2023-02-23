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
const gameProcess_1 = require("./gameProcess");
const checkers_core_1 = require("../build-wasm/checkers_core");
const util = __importStar(require("util"));
// https://github.com/ClassicDP/checkers_core#front-click-handler-1
describe("Game tests", () => {
    test("applyFrontClick", () => {
        let gameProcess = new gameProcess_1.GameProcess(8, checkers_core_1.Color.White);
        gameProcess.insertPiece(0, checkers_core_1.Color.White, true);
        gameProcess.insertPiece(63, checkers_core_1.Color.White, true);
        gameProcess.insertPiece(18, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(43, checkers_core_1.Color.Black, false);
        console.log(gameProcess.position);
        console.log(gameProcess.applyFrontClick(0));
        console.log(gameProcess.applyFrontClick(63));
        console.log(gameProcess.applyFrontClick(0));
        console.log(gameProcess.applyFrontClick(36));
        console.log(gameProcess.applyFrontClick(57));
        let pos = gameProcess.position;
        expect(pos.cells.filter(x => x !== undefined).length).toEqual(2);
        console.log(pos);
    });
    // https://github.com/ClassicDP/checkers_core#front-click-handler
    test("king move applyFrontClick", () => {
        let gameProcess = new gameProcess_1.GameProcess(8, checkers_core_1.Color.White);
        gameProcess.insertPiece(47, checkers_core_1.Color.White, false);
        gameProcess.insertPiece(54, checkers_core_1.Color.Black, true);
        gameProcess.insertPiece(52, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(29, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(13, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(18, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(34, checkers_core_1.Color.Black, false);
        gameProcess.insertPiece(50, checkers_core_1.Color.Black, false);
        console.log(gameProcess.position);
        let move;
        console.log(move = gameProcess.applyFrontClick(47));
        expect(move.list[0].kingMove).toEqual(true);
        console.log(gameProcess.applyFrontClick(61));
        console.log(gameProcess.applyFrontClick(43));
        console.log(gameProcess.applyFrontClick(22));
        console.log(gameProcess.applyFrontClick(4));
        console.log(gameProcess.applyFrontClick(25));
        console.log(gameProcess.applyFrontClick(43));
        console.log(gameProcess.applyFrontClick(57));
        let pos = gameProcess.position;
        expect(pos.cells.filter(x => x !== undefined).length).toEqual(1);
        console.log(pos);
    });
    test("insert and delete pieces", () => {
        let gameProcess = new gameProcess_1.GameProcess(8);
        gameProcess.insertPiece(54, checkers_core_1.Color.White, true);
        gameProcess.insertPiece(9, checkers_core_1.Color.Black, true);
        let state = gameProcess.position.state;
        expect(state.black.king).toEqual(1);
        expect(state.white.king).toEqual(1);
        console.log(state);
        gameProcess.removePiece(54);
        state = gameProcess.position.state;
        expect(state.black.king).toEqual(1);
        expect(state.white.king).toEqual(0);
        console.log(gameProcess.position.state);
        console.log(gameProcess.position.state);
    });
    // https://github.com/ClassicDP/checkers_core#one-of-42-strike-variants
    test("move variants Strike", () => {
        let gameProcess = new gameProcess_1.GameProcess(8);
        gameProcess.insertPiece(0, checkers_core_1.Color.White, true);
        [9, 11, 13, 25, 27, 29, 41, 43, 45].forEach(i => gameProcess.insertPiece(i, checkers_core_1.Color.Black, false));
        console.time('list');
        console.timeEnd('list');
        let list = gameProcess.getMoveList(checkers_core_1.Color.White);
        console.log(list.list.map(x => x.strike.vec));
        expect(list.list.length).toEqual(42);
    });
    // https://github.com/ClassicDP/checkers_core#strike-variants
    test("move variants Strike simple to king and continue", () => {
        let gameProcess = new gameProcess_1.GameProcess(8);
        gameProcess.insertPiece(47, checkers_core_1.Color.White, false);
        gameProcess.insertPiece(63, checkers_core_1.Color.White, false);
        gameProcess.insertPiece(15, checkers_core_1.Color.White, true);
        [54, 43, 20].forEach(i => gameProcess.insertPiece(i, checkers_core_1.Color.Black, false));
        let list = gameProcess.getMoveList(checkers_core_1.Color.White);
        expect(list.list.filter(x => x.strike.vec[0].from == 47)[0].strike.king_move).toEqual(true);
        expect(list.list.filter(x => x.strike.vec[0].from == 63)[0].strike.king_move).toEqual(false);
        console.log(util.inspect(list.list, { depth: 5 }));
        expect(list.list.length).toEqual(5);
    });
    // https://github.com/ClassicDP/checkers_core#move-variants
    test("move variants Quite move", () => {
        let gameProcess = new gameProcess_1.GameProcess(8);
        gameProcess.insertPiece(27, checkers_core_1.Color.White, true);
        [4, 48, 54].forEach(i => gameProcess.insertPiece(i, checkers_core_1.Color.White, false));
        console.log(gameProcess.position);
        let list = gameProcess.getMoveList(checkers_core_1.Color.White);
        console.log(list.list.map(x => x.mov));
        expect(list.list.length).toEqual(15);
    });
    test("performance", () => {
        console.time("test");
        checkers_core_1.PositionEnvironment.game();
        console.timeEnd("test");
    });
});
