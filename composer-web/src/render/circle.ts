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
