import { Align, Justify, TextInstruction } from "./text";

export function justifyToTextAlign(input: Justify) {
  switch (input) {
    case Justify.Middle:
      return "center";
    case Justify.End:
      return "right";
    default:
    case Justify.Start:
      return "left";
  }
}

export function alignToBasline(input: Align) {
  switch (input) {
    case Align.Bottom:
      return "bottom";
    case Align.Top:
      return "top";
    case Align.Middle:
      return "middle";
    default:
  }
}

export function drawText(
  ctx: CanvasRenderingContext2D,
  instruction: TextInstruction,
  space: number
) {
  ctx.fillStyle = instruction.styles.color;
  ctx.font = `${instruction.styles.size * space}px ${instruction.styles.font}`;
  ctx.textAlign = justifyToTextAlign(instruction.styles.justify);
  ctx.textBaseline = alignToBasline(instruction.styles.align);

  ctx.fillText(instruction.value, instruction.x * space, instruction.y * space);
}
