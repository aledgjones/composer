import { RenderInstruction, InstructionType } from "./instructions";

export interface CircleStyles {
  color: string;
}

export type Circle = {
  styles: CircleStyles;
  x: number;
  y: number;
  radius: number;
};
export type CircleInstruction = RenderInstruction<Circle>;

export function buildCircle(
  key: string,
  styles: CircleStyles,
  x: number,
  y: number,
  radius: number
): CircleInstruction {
  return {
    key,
    type: InstructionType.Circle,
    styles,
    x,
    y,
    radius,
  };
}

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
