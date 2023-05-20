import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  let Image = wasm.Image;
  return Comlink.proxy({ Image });
}

Comlink.expose({
  handlers: initHandlers()
});
