export const enum InstructionType {
  Line = "Line",
  Text = "Text",
  Circle = "Circle",
  Curve = "Curve",
  Box = "Box",
  Shape = "Shape",
}

export interface RenderInstructionBase {
  key: string;
  type: InstructionType;
}
export type RenderInstruction<T> = RenderInstructionBase & T;
