import { LineInstruction } from "./line";

export function drawLine(
  ctx: CanvasRenderingContext2D,
  instruction: LineInstruction,
  dpi: number
) {
  ctx.beginPath();
  ctx.lineWidth = instruction.width * dpi;
  ctx.strokeStyle = instruction.color;

  for (let i = 0; i < instruction.points.length; i++) {
    const [x, y] = instruction.points[i];
    if (i === 0) {
      ctx.moveTo(x * dpi, y * dpi);
    } else {
      ctx.lineTo(x * dpi, y * dpi);
    }
  }
  ctx.stroke();
}
