import { RenderInstruction, InstructionType } from "./instructions";

export interface BoxStyles {
  outline?: {
    color: string;
    thickness: number;
  };
  color: string;
}

export type BoxInstruction = RenderInstruction<{
  styles: BoxStyles;
  x: number;
  y: number;
  width: number;
  height: number;
}>;

export function buildBox(
  key: string,
  styles: BoxStyles,
  x: number,
  y: number,
  width: number,
  height: number
): BoxInstruction {
  return {
    key,
    type: InstructionType.Box,
    styles,
    x,
    y,
    width,
    height,
  };
}

export function drawBox(
  ctx: CanvasRenderingContext2D,
  instruction: BoxInstruction
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.styles.color;
  ctx.rect(instruction.x, instruction.y, instruction.width, instruction.height);
  if (instruction.styles.outline) {
    ctx.strokeStyle = instruction.styles.outline.color;
    ctx.lineWidth = instruction.styles.outline.thickness;
    ctx.stroke();
  }
  ctx.fill();
}
