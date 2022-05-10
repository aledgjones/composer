import { RenderInstruction, InstructionType } from "./instructions";

export interface CurveStyles {
  color: string;
}

type CurvePoint = { x: number; y: number; thickness: number };
type CurvePoints = [CurvePoint, CurvePoint, CurvePoint];
export type CurveInstruction = RenderInstruction<{
  styles: CurveStyles;
  points: CurvePoints;
}>;

export function buildCurve(
  key: string,
  styles: CurveStyles,
  ...points: CurvePoints
): CurveInstruction {
  return {
    key,
    type: InstructionType.Curve,
    styles,
    points,
  };
}

export function getControlPoints(
  P0: CurvePoint,
  P1: CurvePoint,
  P2: CurvePoint
) {
  return [
    { x: P0.x, y: P0.y + P0.thickness / 2 },
    {
      x: P1.x,
      y:
        2 * (P1.y + P1.thickness / 2) -
        0.5 * (P0.y + P0.thickness / 2) -
        0.5 * (P2.y + P2.thickness / 2),
    },
    { x: P2.x, y: P2.y + P2.thickness / 2 },
    { x: P2.x, y: P2.y - P2.thickness / 2 },
    {
      x: P1.x,
      y:
        2 * (P1.y - P1.thickness / 2) -
        0.5 * (P0.y + P0.thickness / 2) -
        0.5 * (P2.y - P2.thickness / 2),
    },
    { x: P0.x, y: P0.y - P0.thickness / 2 },
  ];
}

export function drawCurve(
  ctx: CanvasRenderingContext2D,
  instruction: CurveInstruction
) {
  const [P0, P1, P2, P3, P4, P5] = getControlPoints(
    instruction.points[0],
    instruction.points[1],
    instruction.points[2]
  );

  ctx.beginPath();
  ctx.moveTo(P0.x, P0.y);
  ctx.quadraticCurveTo(P1.x, P1.y, P2.x, P2.y);
  ctx.lineTo(P3.x, P3.y);
  ctx.quadraticCurveTo(P4.x, P4.y, P5.x, P5.y);
  ctx.closePath();
  ctx.fillStyle = instruction.styles.color;
  ctx.fill();
}
