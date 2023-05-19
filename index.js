import init, { render } from "./crates/rrt-wasm/pkg/rrt_wasm.js";

const main = async () => {
  const { memory } = await init("./crates/rrt-wasm/pkg/rrt_wasm_bg.wasm");

  const width = 400;
  const height = 225;
  const num_samples = 100;
  const max_bounces = 50;

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');

  const dataArray = render(width, height, num_samples, max_bounces);
  const imageData = new ImageData(dataArray, width);
  ctx.putImageData(imageData, 0, 0);
};

main();
