// fetch_proactor_companion.mjs — stage the upstream IREE proactor companion.
//
// `runtime/iree-wasm-runtime.mjs` imports `./iree_proactor_imports.mjs`, which
// is upstream IREE source (runtime/src/iree/async/platform/js/proactor_imports.mjs).
// It is NOT vendored here so the example never diverges from the pinned runtime.
// This Node helper copies it from a local IREE checkout.
//
// Usage:
//   IREE_SRC=/path/to/iree node web/fetch_proactor_companion.mjs
// or:
//   node web/fetch_proactor_companion.mjs /path/to/iree

import { copyFile, access } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..');

const srcRoot = process.argv[2] || process.env.IREE_SRC;
if (!srcRoot) {
  console.error(
      'Set IREE_SRC (or pass a path) to a pinned IREE source tree.');
  process.exit(2);
}

const from = path.join(
    srcRoot, 'runtime', 'src', 'iree', 'async', 'platform', 'js',
    'proactor_imports.mjs');
const to = path.join(root, 'runtime', 'iree_proactor_imports.mjs');

await access(from);
await copyFile(from, to);
console.log(`Staged proactor companion -> ${path.relative(root, to)}`);
