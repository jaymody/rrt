import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  let render = wasm.render;
  let render_parallel = wasm.render_parallel;
  return Comlink.proxy({ render, render_parallel });
}

Comlink.expose({
  handlers: initHandlers()
});
