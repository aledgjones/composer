import { CurveInstruction, getControlPoints } from "./curve";

export function drawCurve(
  ctx: CanvasRenderingContext2D,
  instruction: CurveInstruction,
  space: number
) {
  const [P0, P1, P2, P3, P4, P5] = getControlPoints(
    instruction.points[0],
    instruction.points[1],
    instruction.points[2]
  );

  ctx.beginPath();
  ctx.moveTo(P0.x * space, P0.y * space);
  ctx.quadraticCurveTo(P1.x * space, P1.y * space, P2.x * space, P2.y * space);
  ctx.lineTo(P3.x * space, P3.y * space);
  ctx.quadraticCurveTo(P4.x * space, P4.y * space, P5.x * space, P5.y * space);
  ctx.closePath();
  ctx.fillStyle = instruction.styles.color;
  ctx.fill();
}
