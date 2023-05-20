import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const width = 400;
const height = 225;

const numSamplesInput = document.getElementById("numSamplesInput");
const maxBouncesInput = document.getElementById("maxBouncesInput");

const renderButton = document.getElementById("renderButton");

const canvas = document.getElementById("canvas");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext('2d');

const timeOutput = document.getElementById("timeOutput");

async function getWasmExports() {
  return await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;
}

(async function init() {
  let { Image } = await getWasmExports();

  const image = await new Image(width, height, maxBouncesInput.value);

  renderButton.onclick = async function () {
    // clear the canvas and indicate we waiting on the render
    ctx.clearRect(0, 0, width, height)
    timeOutput.innerText = "rendering ...";

    // render the image and compute the time it took
    const start = performance.now();
    await image.render(numSamplesInput.value);
    const elapsed = performance.now() - start;

    // update the time output text
    timeOutput.innerText = `${(elapsed / 1000).toFixed(4)}s`;

    // draw the image on the canvas
    const rawImageData = await image.get_image_so_far();
    console.log(rawImageData.length, rawImageData[0]);
    const imageData = new ImageData(rawImageData, width);
    ctx.putImageData(imageData, 0, 0);
  };
})();
