import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  // comlink doesn't seem to work with wasm generated classes, so we use this
  // hack to return a function that creates our image
  let Image = function (width, height, max_bounces) { return wasm.Image.new(width, height, max_bounces); };
  return Comlink.proxy({ Image });
}

Comlink.expose({
  handlers: initHandlers()
});
