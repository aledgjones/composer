import { RenderInstruction } from "./instructions";

export type Circle = {
  color: string;
  radius: number;
  point: [number, number];
};
export type CircleInstruction = RenderInstruction<Circle>;

export function drawCircle(
  ctx: CanvasRenderingContext2D,
  instruction: CircleInstruction
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.color;
  ctx.arc(
    instruction.point[0],
    instruction.point[1],
    instruction.radius,
    0,
    2 * Math.PI
  );
  ctx.fill();
}
