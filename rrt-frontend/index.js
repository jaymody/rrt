import * as Comlink from 'comlink';

// TODO
// probably use import { threads } from 'wasm-feature-detect';
// to determine if render parallel is supported on the browser

const max_samples = 500;

const width = 400;
const height = 400;

const numSamplesPerStepInput = document.getElementById("numSamplesPerStepInput");
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

  const image = await new Image(width, height);

  renderButton.onclick = async function () {
    renderButton.disabled = true;
    let num_samples_per_step = parseInt(numSamplesPerStepInput.value);
    let max_bounces = parseInt(maxBouncesInput.value);

    // clear the canvas and image and indicate we waiting on the render
    ctx.clearRect(0, 0, width, height);
    await image.clear();
    timeOutput.innerText = "rendering ...";

    let totalElapsedTime = 0.0;
    for (let n = num_samples_per_step; n <= max_samples; n += num_samples_per_step) {
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
      console.log(rawImageData.length, rawImageData[0]);
      const imageData = new ImageData(rawImageData, width);
      ctx.putImageData(imageData, 0, 0);
    }

    renderButton.disabled = false;
  };
})();
