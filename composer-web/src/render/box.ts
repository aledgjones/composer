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
