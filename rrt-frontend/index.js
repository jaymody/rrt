import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const max_samples = 500;
const width = 400;
const height = 400;
let redraw = true;

const numSamplesPerStepInput = document.getElementById("numSamplesPerStepInput");
const maxBouncesInput = document.getElementById("maxBouncesInput");
const inputForm = document.getElementById("inputForm");
const timeOutput = document.getElementById("timeOutput");

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

// on input, redraw
inputForm.oninput = async function () {
  redraw = true;
  numSamplesPerStepOutput.innerText = numSamplesPerStepInput.value;
  maxBouncesOutput.innerText = maxBouncesInput.value;
};

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function getWasmExports() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

async function render_loop(image) {
  let n = 0;
  let totalElapsedTime = 0.0;
  let num_samples_per_step = parseInt(numSamplesPerStepInput.value);
  let max_bounces = parseInt(maxBouncesInput.value);

  while (true) {
    if (redraw) {
      // ctx.clearRect(0, 0, width, height);
      await image.clear();

      n = 0;
      redraw = false;
      totalElapsedTime = 0.0;

      num_samples_per_step = parseInt(numSamplesPerStepInput.value);
      max_bounces = parseInt(maxBouncesInput.value);
    }
    else if (n <= max_samples) {
      // render the image and compute the time it took
      const start = performance.now();
      await image.render(num_samples_per_step, max_bounces);
      const elapsed = performance.now() - start;
      totalElapsedTime += elapsed;

      // update the time output text
      const samples_per_second = n / (totalElapsedTime / 1000);
      const fps = (n / num_samples_per_step) / (totalElapsedTime / 1000);
      timeOutput.innerText = `${samples_per_second.toFixed(2)} samples per second, ${fps.toFixed(2)} fps `;

      // draw the image on the canvas
      const rawImageData = await image.get_image_so_far();
      const imageData = new ImageData(rawImageData, width);
      ctx.putImageData(imageData, 0, 0);

      n += num_samples_per_step;
    } else {
      // we sleep here so we don't take up clock cycles with the infinite loop
      await sleep(100);
    }
  }
}

(async function init() {
  let { Image } = await getWasmExports();

  // get wasm image class
  const image = await new Image(width, height);

  // start render loop
  await render_loop(image);
})();
