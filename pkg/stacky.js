/* @ts-self-types="./stacky.d.ts" */
import * as bg from "./stacky_bg.js";

let wasm;

async function init() {
    if (wasm) return wasm;

    const imports = { './stacky_bg.js': bg };
    const url = new URL('./stacky_bg.wasm', import.meta.url);
    const response = await fetch(url);
    const bytes = await response.arrayBuffer();
    const result = await WebAssembly.instantiate(bytes, imports);
    wasm = result.instance.exports;
    bg.__wbg_set_wasm(wasm);
    wasm.__wbindgen_start();
    return wasm;
}

export default init;
export {
    Program, get_top
} from "./stacky_bg.js";
