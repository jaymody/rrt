import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  // Comlink doesn't seem to work with wasm generated classes, so we use this
  // hack to return a function that creates our image (reference to where I found this solution):
  // https://github.com/cerrno/pcisph-wasm/blob/e1e844700d8c32bd652f327040d3bd3b698017e7/wasm-worker.ts#L27
  let Image = function (width, height, max_bounces) { return wasm.Image.new(width, height, max_bounces); };
  return Comlink.proxy({ Image });
}

Comlink.expose({
  handlers: initHandlers()
});
