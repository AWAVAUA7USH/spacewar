import * as wasm from "./spacewar_wasm_bg.wasm";
export * from "./spacewar_wasm_bg.js";
import { __wbg_set_wasm } from "./spacewar_wasm_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
