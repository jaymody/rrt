import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const max_samples = 100;
const width = 300;
const height = 300;
let redraw = true;
let totalRaysDrawn = 0;

const numSamplesPerStepInput = document.getElementById("numSamplesPerStepInput");
const maxBouncesInput = document.getElementById("maxBouncesInput");
const xRotInput = document.getElementById("xRotInput");
const yRotInput = document.getElementById("yRotInput");
const fovInput = document.getElementById("fovInput");
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
  xRotOutput.innerText = xRotInput.value;
  yRotOutput.innerText = yRotInput.value;
  fovOutput.innerText = fovInput.value;
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

async function renderLoop(image) {
  let n = 0;
  let totalElapsedTime = 0.0;
  let numSamplesPerStep = parseInt(numSamplesPerStepInput.value);
  let maxBounces = parseInt(maxBouncesInput.value);
  let xRot = parseInt(xRotInput.value);
  let yRot = parseInt(yRotInput.value);
  let fov = parseInt(fovInput.value);

  while (true) {
    if (redraw) {
      numSamplesPerStep = parseInt(numSamplesPerStepInput.value);
      maxBounces = parseInt(maxBouncesInput.value);
      xRot = parseInt(xRotInput.value);
      yRot = parseInt(yRotInput.value);
      fov = parseInt(fovInput.value);

      n = 0;
      redraw = false;
      totalElapsedTime = 0.0;

      await image.clear();
      await image.set_camera(xRot, yRot, fov);
    }
    else if (n <= max_samples) {
      // render the image and compute the time it took
      const start = performance.now();
      await image.render(numSamplesPerStep, maxBounces);
      const elapsed = performance.now() - start;
      totalElapsedTime += elapsed;

      // update the time output text
      const raysDrawn = width * height * numSamplesPerStep;
      totalRaysDrawn += raysDrawn;

      const samplesPerSecond = n / (totalElapsedTime / 1000);
      const fps = (n / numSamplesPerStep) / (totalElapsedTime / 1000);

      timeOutput.innerText =
        `${samplesPerSecond.toFixed(2)} samples per second
        ${fps.toFixed(2)} frames per second
        ${totalRaysDrawn / 1000000}M total rays cast`;

      // draw the image on the canvas
      const rawImageData = await image.get_image_so_far();
      const imageData = new ImageData(rawImageData, width);
      ctx.putImageData(imageData, 0, 0);

      n += numSamplesPerStep;
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
  renderLoop(image);
})();
