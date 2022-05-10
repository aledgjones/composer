import { FC, useLayoutEffect, useRef } from "react";
import { drawBox } from "../../render/box";
import { drawCircle } from "../../render/circle";
import { drawCurve } from "../../render/curve";
import { InstructionType, RenderInstruction } from "../../render/instructions";
import { drawLine } from "../../render/line";
import { drawShape } from "../../render/shape";
import { drawText } from "../../render/text";

interface Props {
  instructions: RenderInstruction<any>[];
  x: number;
  y: number;
  width: number;
  height: number;
}

export const Canvas: FC<Props> = ({ instructions, x, y, width, height }) => {
  const canvas = useRef<HTMLCanvasElement>(null);

  useLayoutEffect(() => {
    if (canvas?.current) {
      const ctx = canvas.current.getContext("2d");
      const dpi = window.devicePixelRatio;

      if (ctx) {
        // setup canvas
        ctx.canvas.height = height * dpi;
        ctx.canvas.width = width * dpi;
        ctx.canvas.style.height = `${height}px`;
        ctx.canvas.style.width = `${width}px`;

        ctx.setTransform(dpi, 0, 0, dpi, -x * dpi, -y * dpi);

        // clear canvas
        ctx.fillStyle = "#fff";
        ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

        // render instruction set
        instructions.forEach((instruction: RenderInstruction<any>) => {
          switch (instruction.type) {
            case InstructionType.Line:
              drawLine(ctx, instruction);
              break;
            case InstructionType.Text:
              drawText(ctx, instruction);
              break;
            case InstructionType.Circle:
              drawCircle(ctx, instruction);
              break;
            case InstructionType.Curve:
              drawCurve(ctx, instruction);
              break;
            case InstructionType.Shape:
              drawShape(ctx, instruction);
              break;
            case InstructionType.Box:
              drawBox(ctx, instruction);
              break;
            default:
              break;
          }
        });
      }
    }
  }, [instructions]);

  return <canvas ref={canvas} className="renderer__canvas" />;
};
