import init, { render } from "./crates/rrt-wasm/pkg/rrt_wasm.js";

const main = async () => {
  const { memory } = await init("./crates/rrt-wasm/pkg/rrt_wasm_bg.wasm");

  const width = 400;
  const height = 225;

  const canvas = document.getElementById("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');

  const dataArray = render(width, height);
  const imageData = new ImageData(dataArray, width);
  ctx.putImageData(imageData, 0, 0);
};

main();
