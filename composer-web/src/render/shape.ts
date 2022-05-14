import { InstructionType, RenderInstruction } from "./instructions";

type PathPoint = { x: number; y: number };
type Path = PathPoint[];

export type ShapeInstruction = RenderInstruction<{
  type: InstructionType.Shape;
  color: string;
  points: Path;
}>;

export function drawShape(
  ctx: CanvasRenderingContext2D,
  instruction: ShapeInstruction
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.color;
  for (let i = 0; i < instruction.points.length; i++) {
    const { x, y } = instruction.points[i];
    if (i === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  }
  ctx.closePath();
  ctx.fill();
}
