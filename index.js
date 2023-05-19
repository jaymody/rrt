import init, { greet } from "./crates/rrt-wasm/pkg/rrt_wasm.js";

const main = async () => {
  const { memory } = await init("./crates/rrt-wasm/pkg/rrt_wasm_bg.wasm");
  let text = document.getElementById("text");
  text.textContent = greet("World");
};

main();
