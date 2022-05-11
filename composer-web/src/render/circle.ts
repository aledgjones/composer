import { RenderInstruction } from "./instructions";

type CirclePoint = { x: number; y: number };
export type Circle = {
  color: string;
  radius: number;
  point: CirclePoint;
};
export type CircleInstruction = RenderInstruction<Circle>;

export function drawCircle(
  ctx: CanvasRenderingContext2D,
  instruction: CircleInstruction
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.color;
  ctx.arc(
    instruction.point.x,
    instruction.point.y,
    instruction.radius,
    0,
    2 * Math.PI
  );
  ctx.fill();
}
