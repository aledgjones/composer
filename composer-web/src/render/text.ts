import { RenderInstruction, InstructionType } from "./instructions";

export type Text = {
  type: InstructionType.Line;
  value: string;
  x: number;
  y: number;
  color: string;
  font: string;
  size: number;
  justify: CanvasTextAlign;
  align: CanvasTextBaseline;
};

export type TextInstruction = RenderInstruction<Text>;
