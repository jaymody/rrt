import * as Comlink from 'comlink';

const myValue = 42;
class MyClass {
  logSomething() {
    console.log(`myValue=${myValue}`)
  }
}

async function initHandlers() {
  // const wasm = await import('rrt-wasm');
  // await wasm.default(); // initialize wasm
  // await wasm.initialize(navigator.hardwareConcurrency); // initialize with max number of threads avail

  // let Image = wasm.Image;
  // return Comlink.proxy({ Image });
  return Comlink.proxy(MyClass);
}

Comlink.expose({ handlers: initHandlers() });
