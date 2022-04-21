import { getOffscreenCanvas } from "./get-offscreen-canvas";

const canvas = getOffscreenCanvas(0, 0);
const ctx = canvas.getContext("2d", { alpha: false });

const cache: { [key: string]: number } = {};

export function measureText(text: string, size: number, font: string) {
  const key = `${text}, ${size}, ${font}`;
  const value = cache[key];
  if (value) {
    return value;
  } else {
    if (ctx) {
      ctx.font = `${size}px ${font}`;
      const width = ctx.measureText(text).width;
      cache[key] = width;
      return width;
    } else {
      return 0;
    }
  }
}
