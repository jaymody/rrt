import * as Comlink from 'comlink';

async function getWasmExports() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

(async function init() {
  let { Image } = await getWasmExports();

  // the image  gets created and the rust program logs it, so we know rust
  // code is executing correctly here
  let instance = await Image.new(1, 2, 3);

  // error!!!
  // Unhandled Promise Rejection: TypeError: instance.arr_len is not a function.
  // (In 'instance.arr_len()', 'instance.arr_len' is undefined)	(anonymous function)
  console.log(await instance.arr_len());
})();
