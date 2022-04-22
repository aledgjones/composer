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

export function drawText(
  ctx: CanvasRenderingContext2D,
  instruction: TextInstruction,
  dpi: number
) {
  ctx.fillStyle = instruction.color;
  ctx.font = `${instruction.size * dpi}px ${instruction.font}`;
  ctx.textAlign = instruction.justify;
  ctx.textBaseline = instruction.align;

  ctx.fillText(instruction.value, instruction.x * dpi, instruction.y * dpi);
}
