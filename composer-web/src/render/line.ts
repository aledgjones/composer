import { InstructionType, RenderInstruction } from "./instructions";

type PathPoint = { x: number; y: number };
type Path = PathPoint[];

export type LineInstruction = RenderInstruction<{
  type: InstructionType.Line;
  color: string;
  width: number;
  points: Path;
}>;

export function drawLine(
  ctx: CanvasRenderingContext2D,
  instruction: LineInstruction
) {
  ctx.beginPath();
  ctx.lineWidth = instruction.width;
  ctx.strokeStyle = instruction.color;

  for (let i = 0; i < instruction.points.length; i++) {
    const { x, y } = instruction.points[i];
    if (i === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  }
  ctx.stroke();
}
