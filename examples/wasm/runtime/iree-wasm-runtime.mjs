// Supply IREE's proactor and clock imports to an Emscripten module.
// Stage the companion with web/fetch_proactor_companion.mjs before use.

import { createImports } from './iree_proactor_imports.mjs';

function createSyscallImports() {
  if (typeof performance === 'undefined' ||
      typeof performance.now !== 'function') {
    throw new Error('IREE requires a monotonic performance.now() clock');
  }
  return {
    clock_now_ns() {
      const ms = performance.now();
      return BigInt(Math.round(ms * 1e6));
    },
  };
}

// Instantiate an IREE Emscripten module with the IREE host import modules wired.
//
//   emscriptenFactory : the default export of the Emscripten-generated .mjs
//                       (built with -sMODULARIZE=1 -sEXPORT_ES6=1).
//   options.wasmBinary: Uint8Array/ArrayBuffer of the companion .wasm. Required
//                       because we take over instantiation to inject imports.
//   options.moduleOverrides: extra Emscripten Module overrides (arguments,
//                       print, locateFile, etc.).
//
// Returns the initialized Emscripten Module instance.
export async function instantiateIree(emscriptenFactory, options = {}) {
  const { wasmBinary, moduleOverrides = {} } = options;
  if (!wasmBinary) {
    throw new Error(
        'instantiateIree: options.wasmBinary (the IREE .wasm bytes) is required');
  }

  // Shared context threaded into the proactor companion. memory/exports are
  // filled in once the instance exists; the inline proactor reads them lazily.
  const context = { inline: true, memory: null, exports: null };

  const Module = await emscriptenFactory({
    ...moduleOverrides,
    instantiateWasm(info, receiveInstance) {
      info['iree_proactor'] = createImports(context);
      info['iree_syscall'] = createSyscallImports();
      WebAssembly.instantiate(wasmBinary, info).then((output) => {
        context.memory = output.instance.exports.memory;
        context.exports = output.instance.exports;
        receiveInstance(output.instance, output.module);
      });
      return {};
    },
  });

  return Module;
}
