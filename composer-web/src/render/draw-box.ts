import { BoxInstruction } from "./box";

export function drawBox(
  ctx: CanvasRenderingContext2D,
  instruction: BoxInstruction,
  space: number
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.styles.color;
  ctx.rect(
    instruction.x * space,
    instruction.y * space,
    instruction.width * space,
    instruction.height * space
  );
  if (instruction.styles.outline) {
    ctx.strokeStyle = instruction.styles.outline.color;
    ctx.lineWidth = instruction.styles.outline.thickness * space;
    ctx.stroke();
  }
  ctx.fill();
}
