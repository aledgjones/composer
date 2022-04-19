export function getOffscreenCanvas(height: number, width: number) {
  if (typeof OffscreenCanvas !== "undefined") {
    return new OffscreenCanvas(width, height);
  } else {
    const canvas = document.createElement("canvas");
    canvas.height = height;
    canvas.width = width;
    return canvas;
  }
}
