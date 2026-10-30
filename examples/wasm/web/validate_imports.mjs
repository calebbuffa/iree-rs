// Reject unexpected host imports and unresolved IREE symbols in env.
// This allows unused imports to be removed by the linker.
// Usage: node web/validate_imports.mjs [module.wasm]
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const arg = process.argv[2];
const wasmPath = arg
  ? arg
  : fileURLToPath(new URL(
      '../target/wasm32-unknown-emscripten/release/iree_wasm_example.wasm',
      import.meta.url));

const bytes = readFileSync(wasmPath);
const mod = await WebAssembly.compile(bytes);
const imports = WebAssembly.Module.imports(mod);
const byModule = {};
for (const imp of imports) {
  (byModule[imp.module] ??= []).push(`${imp.name}:${imp.kind}`);
}
console.log('=== wasm import modules ===');
for (const [m, names] of Object.entries(byModule)) {
  console.log(`[${m}] (${names.length})`);
  if (m !== 'env' && m !== 'wasi_snapshot_preview1') {
    for (const n of names.sort()) console.log('   ', n);
  }
}

const allowed = {
  iree_proactor: new Set(
      ['ring_drain', 'poll_wait', 'timer_start', 'timer_cancel', 'wake', 'schedule_drain']),
  iree_syscall: new Set(['clock_now_ns']),
};
let ok = true;
for (const [m, names] of Object.entries(byModule)) {
  if (m === 'env' || m === 'wasi_snapshot_preview1') continue;
  if (!allowed[m]) { console.log(`UNEXPECTED import module: ${m}`); ok = false; continue; }
  const got = new Set(names.map((x) => x.split(':')[0]));
  for (const g of got) if (!allowed[m].has(g)) { console.log(`UNEXPECTED ${m}.${g}`); ok = false; }
}
const envNames = (byModule['env'] || []).map((x) => x.split(':')[0]);
const suspicious = envNames.filter(
    (n) => /^(iree_|flatcc_|ring_drain|poll_wait|timer_|wake|schedule_drain|clock_now_ns)/.test(n));
if (suspicious.length) {
  console.log('SUSPICIOUS env imports (possible real missing symbols):', suspicious);
  ok = false;
}
console.log(ok ? 'IMPORT_VALIDATION: PASS' : 'IMPORT_VALIDATION: FAIL');
process.exit(ok ? 0 : 1);
