import * as Comlink from 'comlink';

async function initHandlers() {
  const wasm = await import('rrt-wasm');
  await wasm.default(); // initialize wasm
  await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  let Image = wasm.Image;

  // works just fine!!! prints 8 as expected
  console.log(Image.new(1, 2, 3).arr_len());
  return Comlink.proxy({ Image });
}

Comlink.expose({ handlers: initHandlers() });
