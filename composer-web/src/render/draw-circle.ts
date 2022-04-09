import { CircleInstruction } from "./circle";

export function drawCircle(
  ctx: CanvasRenderingContext2D,
  instruction: CircleInstruction,
  space: number
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.styles.color;
  ctx.arc(
    instruction.x * space,
    instruction.y * space,
    instruction.radius * space,
    0,
    2 * Math.PI
  );
  ctx.fill();
}
