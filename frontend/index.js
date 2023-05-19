import * as Comlink from 'comlink';

const width = 800;
const height = 450;
const num_samples = 100;
const max_bounces = 50;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

(async function init() {
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  const start = performance.now();
  const dataArray = await handlers["fast"](width, height, num_samples, max_bounces);
  const elapsed = performance.now() - start;
  document.getElementById("time").innerText = elapsed / 1000;

  const imageData = new ImageData(dataArray, width);
  ctx.putImageData(imageData, 0, 0);
})();
