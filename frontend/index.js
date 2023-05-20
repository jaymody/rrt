import * as Comlink from 'comlink';

async function getWasmExports() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

(async function init() {
  let MyClass = await getWasmExports();
  let instance = await new MyClass();
  instance.logSomething();
})();
