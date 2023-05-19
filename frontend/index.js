import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const width = 800;
const height = 450;
const num_samples = 100;
const max_bounces = 50;

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

async function getWasmFunctions() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

(async function init() {
  let { render, render_parallel } = await getWasmFunctions();

  const start = performance.now();
  const dataArray = await render_parallel(width, height, num_samples, max_bounces);
  const elapsed = performance.now() - start;
  document.getElementById("time").innerText = elapsed / 1000;

  const imageData = new ImageData(dataArray, width);
  ctx.putImageData(imageData, 0, 0);
})();
