import { InstructionType, RenderInstruction } from "./instructions";

type CoordX = number;
type CoordY = number;
type PathPoint = [CoordX, CoordY];
type Path = PathPoint[];

export type LineInstruction = RenderInstruction<{
  type: InstructionType.Line;
  color: string;
  width: number;
  points: Path;
}>;
