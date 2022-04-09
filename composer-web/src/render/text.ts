import { RenderInstruction, InstructionType } from "./instructions";

export const enum Justify {
  Start,
  Middle,
  End,
}

export const enum Align {
  Top,
  Middle,
  Bottom,
}

export interface TextStyles {
  color: string;
  font: string;
  size: number;
  justify: Justify;
  align: Align;
  lineHeight: number;
}

export type Text = { styles: TextStyles; value: string; x: number; y: number };
export type TextInstruction = RenderInstruction<Text>;

export function buildText(
  key: string,
  styles: TextStyles,
  x: number,
  y: number,
  value: string
): TextInstruction {
  return {
    key,
    type: InstructionType.Text,
    styles,
    x,
    y,
    value,
  };
}
