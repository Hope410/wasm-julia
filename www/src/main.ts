import { greet } from "wasm-pkg";
import _ from "lodash";

function render(ctx: CanvasRenderingContext2D) {
  greet(ctx, "World");
  // requestAnimationFrame(() => render(ctx))
}


(function main() {
  const canvas: HTMLCanvasElement | null = document.querySelector("#canvas");
  if (!canvas) {
    throw new Error("Can't get canvas element");
  }

  const ctx = canvas.getContext("2d");

  if (!ctx) {
    throw new Error("Can't get canvas 2D context");
  }

  render(ctx);
})();
