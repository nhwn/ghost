let imports = {};
let wasm;

let WASM_VECTOR_LEN = 0;

let cachegetNodeBufferMemory0 = null;
function getNodeBufferMemory0() {
    if (cachegetNodeBufferMemory0 === null || cachegetNodeBufferMemory0.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory0 = ;
    }
    return cachegetNodeBufferMemory0;
}

function passStringToWasm0(arg, malloc) {

    const len = Buffer.byteLength(arg);
    // len == 13
    const ptr = malloc(len);
    Buffer.from(wasm.memory.buffer).write(arg, ptr, len);
    WASM_VECTOR_LEN = len;
    return ptr;
}
/**
* @param {string} input
* @returns {boolean}
*/
module.exports.check = function(input) {
    var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.check(ptr0, len0);
    return ret !== 0;
};

const path = require('path').join(__dirname, 'ghost_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

