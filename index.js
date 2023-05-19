import init, { render_fast, render_slow } from "./crates/rrt-wasm/pkg/rrt_wasm.js";

const main = async () => {
  const { memory } = await init("./crates/rrt-wasm/pkg/rrt_wasm_bg.wasm");

  const width = 800;
  const height = 450;
  const num_samples = 100;
  const max_bounces = 50;

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');

  const start = performance.now();
  const dataArray = render_slow(width, height, num_samples, max_bounces);
  const elapsed = performance.now() - start;
  document.getElementById("time").innerText = elapsed / 1000;


  const imageData = new ImageData(dataArray, width);
  ctx.putImageData(imageData, 0, 0);
};

main();
