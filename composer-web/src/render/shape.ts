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

export function drawShape(
  ctx: CanvasRenderingContext2D,
  instruction: ShapeInstruction,
  space: number
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.styles.color;
  for (let i = 0; i < instruction.points.length; i++) {
    const [x, y] = instruction.points[i];
    if (i === 0) {
      ctx.moveTo(x * space, y * space);
    } else {
      ctx.lineTo(x * space, y * space);
    }
  }
  ctx.closePath();
  ctx.fill();
}
