import init, { greet } from "./crates/rrt-wasm/pkg/rrt_wasm.js";

const main = async () => {
  const { memory } = await init("./crates/rrt-wasm/pkg/rrt_wasm_bg.wasm");

  const height = 400;
  const width = 300;

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;

  const ctx = canvas.getContext('2d');
  const imageData = ctx.createImageData(width, height);

  for (let i = 0; i < height; i += 1) {
    for (let j = 0; j < width; j += 1) {
      let idx = (i * width + j) * 4;
      imageData.data[idx + 0] = 255.99 * i / height;
      imageData.data[idx + 1] = 255.99 * j / width;
      imageData.data[idx + 2] = 0;
      imageData.data[idx + 3] = 255;
    }
  }

  ctx.putImageData(imageData, 0, 0);
};

main();
