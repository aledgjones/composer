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
