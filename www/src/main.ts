import { Julia, Range } from "wasm-pkg";
import _ from "lodash";

const ITERATIONS = 64;
const THRESHOLD = 2;

const RANGE_X = [-1.5, 1.5] as const;
const RANGE_X_WIDTH = RANGE_X[1] - RANGE_X[0];

const RANGE_Y = [-1.5, 1.5] as const;
const RANGE_Y_WIDTH = RANGE_Y[1] - RANGE_Y[0];

const SCALE = 256;

const CANVAS_WIDTH = RANGE_X_WIDTH * SCALE;
const CANVAS_HEIGHT = RANGE_Y_WIDTH * SCALE;

function computeColors(width: number) {
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = 1;
  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  const grd = ctx.createLinearGradient(0, 0, width, 0);
  grd.addColorStop(0.00, "#000764");
  grd.addColorStop(0.16, "#2068CB");
  grd.addColorStop(0.42, "#EDFFFF");
  grd.addColorStop(0.6425, "#FFAA00");
  grd.addColorStop(0.8575, "#000200");
  ctx.fillStyle = grd;
  ctx.fillRect(0, 0, width, 1);
  return new Uint8ClampedArray(ctx.getImageData(0, 0, width, 1).data.buffer);
}

function render(julia: Julia, engine = 0) {
  const real = Math.sin(engine);
  const imaginary = Math.cos(engine);

  julia.draw(Range.new(...RANGE_X), Range.new(...RANGE_Y), real, imaginary);

  requestAnimationFrame(() => render(julia, engine + 0.1));
}

(function main() {
  const canvas: HTMLCanvasElement | null = document.querySelector("#canvas");
  if (!canvas) {
    throw new Error("Can't get canvas element");
  }

  canvas.width = CANVAS_WIDTH;
  canvas.height = CANVAS_HEIGHT;

  const ctx = canvas.getContext("2d");

  if (!ctx) {
    throw new Error("Can't get canvas 2D context");
  }

  const colors = computeColors(ITERATIONS);
  const julia = Julia.new(ctx, CANVAS_WIDTH, CANVAS_HEIGHT, ITERATIONS, THRESHOLD, colors);

  render(julia);
})();
