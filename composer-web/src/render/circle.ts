import { RenderInstruction } from "./instructions";

export type Circle = {
  color: string;
  radius: number;
  point: [number, number];
};
export type CircleInstruction = RenderInstruction<Circle>;

export function drawCircle(
  ctx: CanvasRenderingContext2D,
  instruction: CircleInstruction,
  dpi: number
) {
  ctx.beginPath();
  ctx.fillStyle = instruction.color;
  ctx.arc(
    instruction.point[0] * dpi,
    instruction.point[1] * dpi,
    instruction.radius * dpi,
    0,
    2 * Math.PI
  );
  ctx.fill();
}
