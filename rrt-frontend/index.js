import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const maxSamples = 100;
const numSamplesPerStep = 1;
const maxBounces = 5;
const width = 400;
const height = 400;

let yRot = 0;
let xRot = 0;
let redraw = true;
let totalRaysDrawn = 0;

const fovInput = document.getElementById("fovInput");
const inputForm = document.getElementById("inputForm");
const timeOutput = document.getElementById("timeOutput");

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

let isMouseDown = false;

canvas.addEventListener("mousedown", (event) => {
  isMouseDown = true;
})

canvas.addEventListener("mouseup", (event) => {
  isMouseDown = false;
})

document.addEventListener("mousemove", (event) => {
  if (isMouseDown) {
    yRot -= event.movementX;
    xRot += event.movementY;
    xRot = Math.max(Math.min(xRot, 90), -90);
    redraw = true;
  }
})

// on input, redraw
inputForm.oninput = async function () {
  redraw = true;
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
  let fov = parseInt(fovInput.value);

  while (true) {
    if (redraw) {
      fov = parseInt(fovInput.value);

      n = 0;
      redraw = false;
      totalElapsedTime = 0.0;

      await image.clear();
      await image.set_camera(xRot, yRot, fov);
    }
    else if (n <= maxSamples) {
      // render the image and compute the time it took
      const start = performance.now();
      await image.render(numSamplesPerStep, maxBounces);
      const elapsed = performance.now() - start;
      totalElapsedTime += elapsed;

      // update the time output text
      const raysDrawn = width * height * numSamplesPerStep;
      totalRaysDrawn += raysDrawn;

      const fps = (n / numSamplesPerStep) / (totalElapsedTime / 1000);

      timeOutput.innerText =
        `${fps.toFixed(2)} frames per second
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
