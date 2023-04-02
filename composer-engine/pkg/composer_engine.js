let wasm;

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

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = new TextEncoder('utf-8');

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

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* Get a full path to def from partial path
* @param {any} selection
* @returns {any}
*/
export function get_full_path_from_partial(selection) {
    const ret = wasm.get_full_path_from_partial(addHeapObject(selection));
    return takeObject(ret);
}

/**
* Get a tree of instruments from a (possibly incomplete) path
* @param {any} selection
* @returns {any}
*/
export function def_tree(selection) {
    const ret = wasm.def_tree(addHeapObject(selection));
    return takeObject(ret);
}

/**
*/
export function run() {
    wasm.run();
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
export const Articulation = Object.freeze({ None:0,"0":"None",Staccato:1,"1":"Staccato",Staccatissimo:2,"2":"Staccatissimo",Tenuto:3,"3":"Tenuto",StaccatoTenuto:4,"4":"StaccatoTenuto", });
/**
*/
export const NoteDuration = Object.freeze({ Whole:0,"0":"Whole",Half:1,"1":"Half",Quarter:2,"2":"Quarter",Eighth:3,"3":"Eighth",Sixteenth:4,"4":"Sixteenth",ThirtySecond:5,"5":"ThirtySecond",SixtyFourth:6,"6":"SixtyFourth",HudredTwentyEighth:7,"7":"HudredTwentyEighth", });
/**
*/
export const Accidental = Object.freeze({ DoubleSharp:0,"0":"DoubleSharp",Sharp:1,"1":"Sharp",Natural:2,"2":"Natural",Flat:3,"3":"Flat",DoubleFlat:4,"4":"DoubleFlat", });
/**
*/
export const BarlineDrawType = Object.freeze({ Single:0,"0":"Single",Double:1,"1":"Double",EndRepeat:2,"2":"EndRepeat",EndStartRepeat:3,"3":"EndStartRepeat",StartRepeat:4,"4":"StartRepeat",Final:5,"5":"Final", });
/**
*/
export const ClefDrawType = Object.freeze({ Hidden:0,"0":"Hidden",G:1,"1":"G",F:2,"2":"F",C:3,"3":"C",Percussion:4,"4":"Percussion", });
/**
*/
export const KeySignatureMode = Object.freeze({ Major:0,"0":"Major",Minor:1,"1":"Minor", });
/**
*/
export const TimeSignatureDrawType = Object.freeze({ Hidden:0,"0":"Hidden",Regular:1,"1":"Regular",CommonTime:2,"2":"CommonTime",SplitCommonTime:3,"3":"SplitCommonTime",Open:4,"4":"Open", });
/**
*/
export const AutoCountStyle = Object.freeze({ Arabic:0,"0":"Arabic",Roman:1,"1":"Roman", });
/**
*/
export const BracketingApproach = Object.freeze({ None:0,"0":"None",Orchestral:1,"1":"Orchestral",SmallEnsemble:2,"2":"SmallEnsemble", });
/**
*/
export const BracketStyle = Object.freeze({ None:0,"0":"None",Wing:1,"1":"Wing",Line:2,"2":"Line", });
/**
*/
export const LayoutType = Object.freeze({ Score:0,"0":"Score",Part:1,"1":"Part",Custom:2,"2":"Custom", });
/**
*/
export const Expression = Object.freeze({ Natural:0,"0":"Natural",Pizzicato:1,"1":"Pizzicato",Spiccato:2,"2":"Spiccato",Staccato:3,"3":"Staccato",Tremolo:4,"4":"Tremolo",Mute:5,"5":"Mute", });
/**
*/
export const InstrumentType = Object.freeze({ Melodic:0,"0":"Melodic",Percussive:1,"1":"Percussive", });
/**
*/
export const PlayerType = Object.freeze({ Solo:0,"0":"Solo",Section:1,"1":"Section", });
/**
*/
export class Engine {

    static __wrap(ptr) {
        const obj = Object.create(Engine.prototype);
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
        wasm.__wbg_engine_free(ptr);
    }
    /**
    * @param {string} flow_key
    * @param {number} tick
    * @param {number} draw_type
    */
    create_barline(flow_key, tick, draw_type) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_create_barline(this.ptr, ptr0, len0, tick, draw_type);
    }
    /**
    * @param {string} flow_key
    * @param {number} tick
    * @param {number} mode
    * @param {number} offset
    */
    create_key_signature(flow_key, tick, mode, offset) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_create_key_signature(this.ptr, ptr0, len0, tick, mode, offset);
    }
    /**
    * @param {string} flow_key
    * @param {number} tick
    * @param {number} beats
    * @param {number} beat_type
    * @param {number} draw_type
    * @param {Uint8Array | undefined} groupings
    */
    create_time_signature(flow_key, tick, beats, beat_type, draw_type, groupings) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(groupings) ? 0 : passArray8ToWasm0(groupings, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        wasm.engine_create_time_signature(this.ptr, ptr0, len0, tick, beats, beat_type, draw_type, ptr1, len1);
    }
    /**
    * Create a tone
    * @param {string} track_key
    * @param {number} tick
    * @param {number} duration
    * @param {number} pitch
    * @param {number | undefined} accidental
    * @param {number} velocity
    * @param {number} articulation
    * @returns {string}
    */
    create_tone(track_key, tick, duration, pitch, accidental, velocity, articulation) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_create_tone(retptr, this.ptr, ptr0, len0, tick, duration, pitch, isLikeNone(accidental) ? 5 : accidental, velocity, articulation);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * update tone pitch
    * @param {string} track_key
    * @param {string} entry_key
    * @param {number} pitch
    */
    set_tone_pitch(track_key, entry_key, pitch) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(entry_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_set_tone_pitch(this.ptr, ptr0, len0, ptr1, len1, pitch);
    }
    /**
    * update tone duration
    * @param {string} track_key
    * @param {string} entry_key
    * @param {number} duration
    */
    set_tone_duration(track_key, entry_key, duration) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(entry_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_set_tone_duration(this.ptr, ptr0, len0, ptr1, len1, duration);
    }
    /**
    * move the tone
    * @param {string} track_key
    * @param {string} entry_key
    * @param {number} new_tick
    */
    shift_tone(track_key, entry_key, new_tick) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(entry_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_shift_tone(this.ptr, ptr0, len0, ptr1, len1, new_tick);
    }
    /**
    * Remove the tone
    * @param {string} track_key
    * @param {string} entry_key
    */
    remove_tone(track_key, entry_key) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(entry_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_remove_tone(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * Slice a tone
    * @param {string} track_key
    * @param {string} entry_key
    * @param {number} slice_at
    */
    slice_tone(track_key, entry_key, slice_at) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(entry_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_slice_tone(this.ptr, ptr0, len0, ptr1, len1, slice_at);
    }
    /**
    * @param {string} track_key
    * @returns {any}
    */
    get_tones(track_key) {
        const ptr0 = passStringToWasm0(track_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_tones(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string} flow_key
    * @param {string} instrument_key
    * @returns {any}
    */
    get_all_tones(flow_key, instrument_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_all_tones(this.ptr, ptr0, len0, ptr1, len1);
        return takeObject(ret);
    }
    /**
    * @param {string} flow_key
    * @param {number} px_per_mm
    * @param {Function} measure
    * @returns {any}
    */
    render(flow_key, px_per_mm, measure) {
        try {
            const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.engine_render(this.ptr, ptr0, len0, px_per_mm, addBorrowedObject(measure));
            return takeObject(ret);
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @returns {number}
    */
    get auto_count_style_solo() {
        const ret = wasm.engine_auto_count_style_solo(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} value
    */
    set auto_count_style_solo(value) {
        wasm.engine_set_auto_count_style_solo(this.ptr, value);
    }
    /**
    * @returns {number}
    */
    get auto_count_style_section() {
        const ret = wasm.engine_auto_count_style_section(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} value
    */
    set auto_count_style_section(value) {
        wasm.engine_set_auto_count_style_section(this.ptr, value);
    }
    /**
    * @param {number} layout_type
    * @param {string} name
    */
    create_engrave(layout_type, name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_create_engrave(this.ptr, layout_type, ptr0, len0);
    }
    /**
    * @returns {any}
    */
    get engraves() {
        const ret = wasm.engine_engraves(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} key
    * @returns {string}
    */
    engrave_name(key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_engrave_name(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} key
    * @returns {boolean}
    */
    get_systemic_barline_single_instrument_system(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_systemic_barline_single_instrument_system(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} key
    * @param {boolean} value
    */
    set_systemic_barline_single_instrument_system(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_systemic_barline_single_instrument_system(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_bracketing_approach(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_bracketing_approach(this.ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_bracketing_approach(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_bracketing_approach(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_bracket_style(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_bracket_style(this.ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_bracket_style(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_bracket_style(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {boolean}
    */
    get_bracket_single_staves(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_bracket_single_staves(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} key
    * @param {boolean} value
    */
    set_bracket_single_staves(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_bracket_single_staves(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {boolean}
    */
    get_sub_bracket(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_sub_bracket(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} key
    * @param {boolean} value
    */
    set_sub_bracket(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_sub_bracket(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_base_note_space(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_base_note_space(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_base_note_space(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_base_note_space(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_minimum_note_space(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_minimum_note_space(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_minimum_note_space(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_minimum_note_space(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_minimum_tie_space(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_minimum_tie_space(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_minimum_tie_space(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_minimum_tie_space(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_note_space_ratio(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_note_space_ratio(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_note_space_ratio(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_note_space_ratio(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} key
    * @returns {number}
    */
    get_space(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_space(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} key
    * @param {number} value
    */
    set_space(key, value) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_space(this.ptr, ptr0, len0, value);
    }
    /**
    * @returns {string}
    */
    create_flow() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_create_flow(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} flow_key
    */
    remove_flow(flow_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_remove_flow(this.ptr, ptr0, len0);
    }
    /**
    * @param {number} old_index
    * @param {number} new_index
    */
    reorder_flow(old_index, new_index) {
        wasm.engine_reorder_flow(this.ptr, old_index, new_index);
    }
    /**
    * @param {string} flow_key
    * @param {string} name
    */
    rename_flow(flow_key, name) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_rename_flow(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} flow_key
    * @returns {number}
    */
    get_flow_length(flow_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_flow_length(this.ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {string} flow_key
    * @param {number} length
    */
    set_flow_length(flow_key, length) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_flow_length(this.ptr, ptr0, len0, length);
    }
    /**
    *
    *     * Assign a player to a flow
    *
    * @param {string} flow_key
    * @param {string} player_key
    */
    assign_player_to_flow(flow_key, player_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_assign_player_to_flow(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    *
    *     * Assign instrument to flow
    *
    * @param {string} flow_key
    * @param {string} instrument_key
    */
    assign_instrument_to_flow(flow_key, instrument_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_assign_instrument_to_flow(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} flow_key
    * @param {string} player_key
    */
    unassign_player_from_flow(flow_key, player_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_unassign_player_from_flow(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} flow_key
    * @param {string} instrument_key
    */
    unassign_instrument_from_flow(flow_key, instrument_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_unassign_instrument_from_flow(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @returns {any}
    */
    get flows() {
        const ret = wasm.engine_flows(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} flow_key
    * @returns {number}
    */
    get_flow_subdivisions(flow_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_flow_subdivisions(this.ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {string} flow_key
    * @returns {string}
    */
    get_flow_title(flow_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_get_flow_title(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} flow_key
    * @param {string} player_key
    * @returns {boolean}
    */
    flow_contains_player(flow_key, player_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.engine_flow_contains_player(this.ptr, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    /**
    * @param {string} flow_key
    * @returns {any}
    */
    get_flow_ticks(flow_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_flow_ticks(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string} flow_key
    * @param {number} at
    * @returns {string}
    */
    get_timestamp(flow_key, at) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_get_timestamp(retptr, this.ptr, ptr0, len0, at);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Create an instrument
    * @param {string} id
    * @returns {string}
    */
    create_instrument(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_create_instrument(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} instrument_key
    */
    remove_instrument(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_remove_instrument(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} instrument_key
    * @returns {string}
    */
    get_instrument_name(instrument_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_get_instrument_name(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} instrument_key
    * @returns {string}
    */
    get_instrument_id(instrument_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_get_instrument_id(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} instrument_key
    * @returns {number}
    */
    get_instrument_volume(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_instrument_volume(this.ptr, ptr0, len0);
        return ret;
    }
    /**
    * @param {string} instrument_key
    * @param {number} value
    */
    set_instrument_volume(instrument_key, value) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_instrument_volume(this.ptr, ptr0, len0, value);
    }
    /**
    * @param {string} instrument_key
    * @returns {boolean}
    */
    get_instrument_solo(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_instrument_solo(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} instrument_key
    */
    toggle_instrument_solo(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_toggle_instrument_solo(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} instrument_key
    * @returns {boolean}
    */
    get_instrument_mute(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_instrument_mute(this.ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * @param {string} instrument_key
    */
    toggle_instrument_mute(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_toggle_instrument_mute(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} instrument_key
    * @returns {any}
    */
    get_instrument_staves(instrument_key) {
        const ptr0 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_instrument_staves(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string} flow_key
    * @param {string} instrument_key
    * @returns {any}
    */
    get_instrument_tracks(flow_key, instrument_key) {
        const ptr0 = passStringToWasm0(flow_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_instrument_tracks(this.ptr, ptr0, len0, ptr1, len1);
        return takeObject(ret);
    }
    /**
    */
    calculate_counts() {
        wasm.engine_calculate_counts(this.ptr);
    }
    /**
    * @returns {string}
    */
    get application_version() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_application_version(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set application_version(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_application_version(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get title() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_title(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set title(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_title(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get subtitle() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_subtitle(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set subtitle(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_subtitle(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get composer() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_composer(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set composer(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_composer(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get arranger() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_arranger(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set arranger(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_arranger(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get lyricist() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_lyricist(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set lyricist(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_lyricist(this.ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get copyright() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_copyright(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} value
    */
    set copyright(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_set_copyright(this.ptr, ptr0, len0);
    }
    /**
    * @returns {number}
    */
    get created() {
        const ret = wasm.engine_created(this.ptr);
        return ret;
    }
    /**
    * @param {number} value
    */
    set created(value) {
        wasm.engine_set_created(this.ptr, value);
    }
    /**
    * @param {number} player_type
    * @returns {string}
    */
    create_player(player_type) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_create_player(retptr, this.ptr, player_type);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} player_key
    */
    remove_player(player_key) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_remove_player(this.ptr, ptr0, len0);
    }
    /**
    * @param {number} from
    * @param {number} to
    */
    reorder_players(from, to) {
        wasm.engine_reorder_players(this.ptr, from, to);
    }
    /**
    * @param {string} player_key
    * @param {string} instrument_key
    */
    assign_instrument_to_player(player_key, instrument_key) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_assign_instrument_to_player(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} player_key
    * @param {string} instrument_key
    */
    unassign_instrument_from_player(player_key, instrument_key) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(instrument_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.engine_unassign_instrument_from_player(this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @param {string} player_key
    * @param {number} from
    * @param {number} to
    */
    reorder_player_instruments(player_key, from, to) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.engine_reorder_player_instruments(this.ptr, ptr0, len0, from, to);
    }
    /**
    * @returns {any}
    */
    get players() {
        const ret = wasm.engine_players(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} player_key
    * @returns {number}
    */
    get_player_type(player_key) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_player_type(this.ptr, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {string} player_key
    * @returns {string}
    */
    get_player_name(player_key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.engine_get_player_name(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @param {string} player_key
    * @returns {any}
    */
    get_player_instruments(player_key) {
        const ptr0 = passStringToWasm0(player_key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.engine_get_player_instruments(this.ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    */
    constructor() {
        const ret = wasm.engine_new();
        return Engine.__wrap(ret);
    }
    /**
    * @param {Function} cb
    */
    listen(cb) {
        wasm.engine_listen(this.ptr, addHeapObject(cb));
    }
    /**
    * @returns {any}
    */
    export() {
        const ret = wasm.engine_export(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} state
    */
    import(state) {
        wasm.engine_import(this.ptr, addHeapObject(state));
    }
    /**
    * @returns {string}
    */
    get state() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.engine_state(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
}
/**
*/
export class Pitch {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pitch_free(ptr);
    }
    /**
    * @returns {number}
    */
    get int() {
        const ret = wasm.__wbg_get_pitch_int(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set int(arg0) {
        wasm.__wbg_set_pitch_int(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get accidental() {
        const ret = wasm.__wbg_get_pitch_accidental(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set accidental(arg0) {
        wasm.__wbg_set_pitch_accidental(this.ptr, arg0);
    }
}
/**
*/
export class Velocity {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_velocity_free(ptr);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_log_608cdc9ac4971012 = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getwithrefkey_5e6d9547403deab8 = function(arg0, arg1) {
        const ret = getObject(arg0)[getObject(arg1)];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_841ac57cff3d672b = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_randomFillSync_6894564c2c334c42 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_805f1c3d65988a5a = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_crypto_e1d53a1d73fb10b8 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_process_038c26bf42b093f8 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_ab37218d2f0b24a8 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_080f4b19d15bc1fe = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_msCrypto_6e7d3e1f92610cbb = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_78a3dcfbdba9cbce = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_get_27fe3dac1c4d0224 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_e498fbc24f9c1d4f = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_b525de17f44a8943 = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_f841cc6f2098f4b5 = function() {
        const ret = new Map();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_b7d530c04fd8b217 = function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_next_88560ec06a094dea = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_1ebec03bbd919843 = function(arg0) {
        const ret = getObject(arg0).done;
        return ret;
    };
    imports.wbg.__wbg_value_6ac8da5cc5b3efda = function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_iterator_55f114446221aa5a = function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_baf4855f9a986186 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_f9876326328f45ed = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_17224bc548dd1d7b = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_isArray_39d28997bf6b96b4 = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_a69f02ee4c4f5065 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_call_9495de66fdbe016b = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_99043a1e2a9e5916 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_388c4c6422704173 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_isSafeInteger_8c4789029e885159 = function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_now_931686b195a14f9d = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_entries_4e1315b774245952 = function(arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_01cebe79ca606cca = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_b56c882b57805732 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_7526649b91a252a6 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('composer_engine_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
