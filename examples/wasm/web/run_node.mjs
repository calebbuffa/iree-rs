// Run an Emscripten-built module. A model failure is a failing process exit.
// Usage: node web/run_node.mjs [artifactDir] [jsName]

import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..');

const artifactDir = process.argv[2] ||
    path.resolve(root, 'target/wasm32-unknown-emscripten/release');
const jsName = process.argv[3] || 'iree_wasm_example.js';
const jsPath = path.join(artifactDir, jsName);
const wasmPath = jsPath.replace(/\.m?js$/, '.wasm');

const { instantiateIree } =
    await import(pathToFileURL(path.join(root, 'runtime', 'iree-wasm-runtime.mjs')));

const factoryModule = await import(pathToFileURL(jsPath));
const emscriptenFactory = factoryModule.default;
const wasmBinary = await readFile(wasmPath);

console.log(`[iree-wasm-example] instantiating ${path.basename(wasmPath)} ...`);
const Module = await instantiateIree(emscriptenFactory, { wasmBinary });
console.log('[iree-wasm-example] instantiated: build + link + init succeeded.');

const code = Module.ccall('iree_wasm_demo_run', 'number', [], []);
if (code === 0) {
  console.log('[iree-wasm-example] model executed on wasm (status 0).');
} else {
  console.error(`[iree-wasm-example] model execution failed (status ${code}).`);
  process.exitCode = 1;
}
