import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const width = 400;
const height = 225;

const numSamplesInput = document.getElementById("numSamplesInput");
const numBouncesInput = document.getElementById("numBouncesInput");

const renderButton = document.getElementById("renderButton");
const renderParallelButton = document.getElementById("renderParallelButton");

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

const timeOutput = document.getElementById("timeOutput");

async function getWasmFunctions() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

(async function init() {
  let { render, render_parallel } = await getWasmFunctions();

  canvas.width = width;
  canvas.height = height;

  async function draw(render_fn) {
    // clear the canvas and indicate we waiting on the render
    ctx.clearRect(0, 0, width, height)
    timeOutput.innerText = "rendering ...";

    // render the image and compute the time it took
    const start = performance.now();
    const dataArray = await render_fn(width, height, numSamplesInput.value, numBouncesInput.value);
    const elapsed = performance.now() - start;

    // update the time output text
    timeOutput.innerText = `${(elapsed / 1000).toFixed(4)}s`;

    // draw the image on the canvas
    const imageData = new ImageData(dataArray, width);
    ctx.putImageData(imageData, 0, 0);
  }

  renderButton.onclick = function () { draw(render) };
  renderParallelButton.onclick = function () { draw(render_parallel) };
})();
