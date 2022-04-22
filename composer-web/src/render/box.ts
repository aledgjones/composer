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
