import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  let render = wasm.render;
  return Comlink.proxy({ render });
}

Comlink.expose({
  handlers: initHandlers()
});
