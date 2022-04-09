import { ShapeInstruction } from "./shape";

export function drawShape(
  ctx: CanvasRenderingContext2D,
  instruction: ShapeInstruction,
  space: number
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.styles.color;
  for (let i = 0; i < instruction.points.length; i++) {
    const [x, y] = instruction.points[i];
    if (i === 0) {
      ctx.moveTo(x * space, y * space);
    } else {
      ctx.lineTo(x * space, y * space);
    }
  }
  ctx.closePath();
  ctx.fill();
}
