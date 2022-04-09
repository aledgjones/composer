import { RenderInstruction, InstructionType } from "./instructions";

export interface ShapeStyles {
  color: string;
}

type CoordX = number;
type CoordY = number;
type PathPoint = [CoordX, CoordY];
type Path = PathPoint[];
export type ShapeInstruction = RenderInstruction<{
  styles: ShapeStyles;
  points: Path;
}>;

export function buildShape(
  key: string,
  styles: ShapeStyles,
  ...points: Path
): ShapeInstruction {
  return {
    key,
    type: InstructionType.Shape,
    styles,
    points,
  };
}
