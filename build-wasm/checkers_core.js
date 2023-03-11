let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder, TextEncoder } = require(`util`);

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function _assertBigInt(n) {
    if (typeof(n) !== 'bigint') throw new Error('expected a bigint argument');
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
module.exports.FinishType = Object.freeze({ Draw1:0,"0":"Draw1",Draw2:1,"1":"Draw2",Draw3:2,"2":"Draw3",Draw4:3,"3":"Draw4",Draw5:4,"4":"Draw5",BlackWin:5,"5":"BlackWin",WhiteWin:6,"6":"WhiteWin", });
/**
*/
module.exports.Color = Object.freeze({ Black:0,"0":"Black",White:1,"1":"White", });
/**
*/
class BestPos {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        const obj = Object.create(BestPos.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bestpos_free(ptr);
    }
}
module.exports.BestPos = BestPos;
/**
*/
class Game {

    static __wrap(ptr) {
        const obj = Object.create(Game.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_game_free(ptr);
    }
    /**
    * @param {number} size
    */
    constructor(size) {
        _assertNum(size);
        const ret = wasm.game_new(size);
        return Game.__wrap(ret);
    }
    /**
    * @param {number} depth
    */
    set_depth(depth) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(depth);
        wasm.game_set_depth(this.ptr, depth);
    }
    /**
    * @param {Piece} piece
    */
    insert_piece(piece) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertClass(piece, Piece);
        if (piece.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        var ptr0 = piece.__destroy_into_raw();
        wasm.game_insert_piece(this.ptr, ptr0);
    }
    /**
    * @param {number} pos
    * @returns {boolean}
    */
    remove_piece(pos) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(pos);
        const ret = wasm.game_remove_piece(this.ptr, pos);
        return ret !== 0;
    }
    /**
    * @returns {any}
    */
    get position() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_position(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {BestPos} pos
    */
    make_move_by_pos_item(pos) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertClass(pos, BestPos);
        if (pos.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.game_make_move_by_pos_item(this.ptr, pos.ptr);
    }
    /**
    * @param {MoveItem} move_item
    */
    make_move_by_move_item(move_item) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertClass(move_item, MoveItem);
        if (move_item.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.game_make_move_by_move_item(this.ptr, move_item.ptr);
    }
    /**
    * @param {number} max_depth
    * @param {number} best_white
    * @param {number} best_black
    * @param {number} depth
    * @returns {BestPos}
    */
    best_move(max_depth, best_white, best_black, depth) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(max_depth);
        _assertNum(best_white);
        _assertNum(best_black);
        _assertNum(depth);
        const ret = wasm.game_best_move(this.ptr, max_depth, best_white, best_black, depth);
        return BestPos.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    get_best_move() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_get_best_move(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {BestPos} pos
    */
    make_best_move(pos) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertClass(pos, BestPos);
        if (pos.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        wasm.game_make_best_move(this.ptr, pos.ptr);
    }
    /**
    * @returns {any}
    */
    find_and_make_best_move_ts_n() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_find_and_make_best_move_ts_n(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {any}
    */
    get_board_list_ts_n() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_get_board_list_ts_n(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {number} i
    * @returns {any}
    */
    move_by_index_ts_n(i) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(i);
        const ret = wasm.game_move_by_index_ts_n(this.ptr, i);
        return takeObject(ret);
    }
    /**
    * @returns {BestPos}
    */
    get_best_move_rust() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_get_best_move_rust(this.ptr);
        return BestPos.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    state_() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            wasm.game_state_(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {number} pack_index
    * @returns {number}
    */
    to_board(pack_index) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(pack_index);
        const ret = wasm.game_to_board(this.ptr, pack_index);
        return ret >>> 0;
    }
    /**
    * @param {number} board_index
    * @returns {number}
    */
    to_pack(board_index) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(board_index);
        const ret = wasm.game_to_pack(this.ptr, board_index);
        return ret >>> 0;
    }
    /**
    * @returns {any}
    */
    get_move_list_for_front() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_get_move_list_for_front(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {any}
    */
    get moveColor() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_get_color(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {number} color
    */
    set moveColor(color) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(color);
        wasm.game_set_color(this.ptr, color);
    }
    /**
    * @returns {number | undefined}
    */
    finish_check() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.game_finish_check(this.ptr);
        return ret === 7 ? undefined : ret;
    }
    /**
    * @param {any} pos_chain
    * @returns {any}
    */
    make_move_for_front(pos_chain) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            wasm.game_make_move_for_front(retptr, this.ptr, addBorrowedObject(pos_chain));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.Game = Game;
/**
*/
class MoveItem {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_moveitem_free(ptr);
    }
}
module.exports.MoveItem = MoveItem;
/**
*/
class Piece {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        const obj = Object.create(Piece.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_piece_free(ptr);
    }
    /**
    * @param {number} pos
    * @param {number} color
    * @param {boolean} is_king
    * @returns {Piece}
    */
    static new(pos, color, is_king) {
        _assertNum(pos);
        _assertNum(color);
        _assertBoolean(is_king);
        const ret = wasm.piece_new(pos, color, is_king);
        return Piece.__wrap(ret);
    }
    /**
    * @param {any} js
    * @returns {Piece | undefined}
    */
    static new_fom_js(js) {
        const ret = wasm.piece_new_fom_js(addHeapObject(js));
        return ret === 0 ? undefined : Piece.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    get it() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        const ptr = this.__destroy_into_raw();
        _assertNum(ptr);
        const ret = wasm.piece_it(ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} js
    */
    set it(js) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        wasm.piece_set_it(this.ptr, addHeapObject(js));
    }
    /**
    * @returns {number}
    */
    get pos() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.__wbg_get_piece_pos(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set pos(arg0) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(arg0);
        wasm.__wbg_set_piece_pos(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get color() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.__wbg_get_piece_color(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set color(arg0) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(arg0);
        wasm.__wbg_set_piece_color(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get is_king() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.__wbg_get_piece_is_king(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set is_king(arg0) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertBoolean(arg0);
        wasm.__wbg_set_piece_is_king(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get stricken() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.__wbg_get_piece_stricken(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set stricken(arg0) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertBoolean(arg0);
        wasm.__wbg_set_piece_stricken(this.ptr, arg0);
    }
}
module.exports.Piece = Piece;
/**
*/
class PositionEnvironment {

    static __wrap(ptr) {
        const obj = Object.create(PositionEnvironment.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_positionenvironment_free(ptr);
    }
    /**
    * @returns {number}
    */
    get size() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.__wbg_get_positionenvironment_size(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set size(arg0) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(arg0);
        wasm.__wbg_set_positionenvironment_size(this.ptr, arg0);
    }
    /**
    * @param {number} size
    */
    constructor(size) {
        _assertNum(size);
        const ret = wasm.positionenvironment_new(size);
        return PositionEnvironment.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    js() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        const ret = wasm.positionenvironment_js(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {Piece} piece
    * @param {number} pos
    * @returns {boolean}
    */
    is_king_move_for(piece, pos) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertClass(piece, Piece);
        if (piece.ptr === 0) {
            throw new Error('Attempt to use a moved value');
        }
        _assertNum(pos);
        const ret = wasm.positionenvironment_is_king_move_for(this.ptr, piece.ptr, pos);
        return ret !== 0;
    }
    /**
    */
    static game() {
        wasm.positionenvironment_game();
    }
    /**
    * @returns {any}
    */
    static test() {
        const ret = wasm.positionenvironment_test();
        return takeObject(ret);
    }
}
module.exports.PositionEnvironment = PositionEnvironment;
/**
*/
class PositionHistoryItem {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_positionhistoryitem_free(ptr);
    }
}
module.exports.PositionHistoryItem = PositionHistoryItem;
/**
*/
class StraightStrike {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_straightstrike_free(ptr);
    }
}
module.exports.StraightStrike = StraightStrike;

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

module.exports.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbg_log_7529978016e706d9 = function() { return logError(function (arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
}, arguments) };

module.exports.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    _assertNum(ret);
    return ret;
};

module.exports.__wbindgen_is_string = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_in = function(arg0, arg1) {
    const ret = getObject(arg0) in getObject(arg1);
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_is_bigint = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'bigint';
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_bigint_from_u64 = function(arg0) {
    const ret = BigInt.asUintN(64, arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_jsval_eq = function(arg0, arg1) {
    const ret = getObject(arg0) === getObject(arg1);
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    if (!isLikeNone(ret)) {
        _assertNum(ret);
    }
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

module.exports.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};

module.exports.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
    const ret = getObject(arg0) == getObject(arg1);
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbg_String_91fba7ded13ba54c = function() { return logError(function (arg0, arg1) {
    const ret = String(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

module.exports.__wbg_getwithrefkey_15c62c2b8546208d = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0)[getObject(arg1)];
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_set_20cbc34131e76824 = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
}, arguments) };

module.exports.__wbg_randomFillSync_6894564c2c334c42 = function() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };

module.exports.__wbg_getRandomValues_805f1c3d65988a5a = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

module.exports.__wbg_crypto_e1d53a1d73fb10b8 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_process_038c26bf42b093f8 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_versions_ab37218d2f0b24a8 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_node_080f4b19d15bc1fe = function() { return logError(function (arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_require_78a3dcfbdba9cbce = function() { return handleError(function () {
    const ret = module.require;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    _assertBoolean(ret);
    return ret;
};

module.exports.__wbg_msCrypto_6e7d3e1f92610cbb = function() { return logError(function (arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_get_27fe3dac1c4d0224 = function() { return logError(function (arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_length_e498fbc24f9c1d4f = function() { return logError(function (arg0) {
    const ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

module.exports.__wbg_new_b525de17f44a8943 = function() { return logError(function () {
    const ret = new Array();
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_newnoargs_2b8b6bd7753c76ba = function() { return logError(function (arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_next_b7d530c04fd8b217 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_next_88560ec06a094dea = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).next();
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_done_1ebec03bbd919843 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).done;
    _assertBoolean(ret);
    return ret;
}, arguments) };

module.exports.__wbg_value_6ac8da5cc5b3efda = function() { return logError(function (arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_iterator_55f114446221aa5a = function() { return logError(function () {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_get_baf4855f9a986186 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_new_f9876326328f45ed = function() { return logError(function () {
    const ret = new Object();
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_set_17224bc548dd1d7b = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
}, arguments) };

module.exports.__wbg_instanceof_ArrayBuffer_a69f02ee4c4f5065 = function() { return logError(function (arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof ArrayBuffer;
    } catch {
        result = false;
    }
    const ret = result;
    _assertBoolean(ret);
    return ret;
}, arguments) };

module.exports.__wbg_call_9495de66fdbe016b = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_isSafeInteger_8c4789029e885159 = function() { return logError(function (arg0) {
    const ret = Number.isSafeInteger(getObject(arg0));
    _assertBoolean(ret);
    return ret;
}, arguments) };

module.exports.__wbg_entries_4e1315b774245952 = function() { return logError(function (arg0) {
    const ret = Object.entries(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_buffer_cf65c07de34b9a08 = function() { return logError(function (arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_new_537b7341ce90bb31 = function() { return logError(function (arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_set_17499e8aa4003ebd = function() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
}, arguments) };

module.exports.__wbg_length_27a2afe8ab42b09f = function() { return logError(function (arg0) {
    const ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

module.exports.__wbg_instanceof_Uint8Array_01cebe79ca606cca = function() { return logError(function (arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Uint8Array;
    } catch {
        result = false;
    }
    const ret = result;
    _assertBoolean(ret);
    return ret;
}, arguments) };

module.exports.__wbg_newwithlength_b56c882b57805732 = function() { return logError(function (arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_subarray_7526649b91a252a6 = function() { return logError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
    const v = getObject(arg1);
    const ret = typeof(v) === 'bigint' ? v : undefined;
    if (!isLikeNone(ret)) {
        _assertBigInt(ret);
    }
    getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

module.exports.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};

const path = require('path').join(__dirname, 'checkers_core_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

