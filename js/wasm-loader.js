import init, * as wasm from '../pkg/academia_wasm.js';

let wasmReady = null;

export async function loadWasm() {
  if (!wasmReady) {
    wasmReady = init().then(() => {
      console.log('🦀 WASM cargado. Versión:', wasm.wasm_version());
      return wasm;
    });
  }
  return wasmReady;
}

export { wasm };
