import { getOffscreenCanvas } from "./get-offscreen-canvas";

const canvas = getOffscreenCanvas(0, 0);
const ctx = canvas.getContext("2d", { alpha: false });

export function measureText(text: string, size: number, font: string) {
  if (ctx) {
    ctx.font = `${size}px ${font}`;
    return ctx.measureText(text).width;
  } else {
    return 0;
  }
}
