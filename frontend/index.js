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

  const image = await Image.new(width, height, maxBouncesInput.value);
  const ptr = await image.get_ptr();
  const rawImageData = new Uint8Array(memory.buffer, ptr, width * height * 4);

  renderButton.onclick = async function () {
    // clear the canvas and indicate we waiting on the render
    ctx.clearRect(0, 0, width, height)
    timeOutput.innerText = "rendering ...";

    // render the image and compute the time it took
    const start = performance.now();
    image.render(numSamplesInput.value);
    const elapsed = performance.now() - start;

    // update the time output text
    timeOutput.innerText = `${(elapsed / 1000).toFixed(4)}s`;

    // draw the image on the canvas
    const imageData = new ImageData(dataArray, width);
    ctx.putImageData(imageData, 0, 0);
  };
})();
