import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initThreadPool(navigator.hardwareConcurrency); // initialize rayon thread pool with max number of threads avail

  return Comlink.proxy({ "slow": wasm.render_slow, "fast": wasm.render_fast });
}

Comlink.expose({
  handlers: initHandlers()
});
