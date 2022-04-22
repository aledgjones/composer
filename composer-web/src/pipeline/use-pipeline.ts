import { RefObject, useLayoutEffect } from "react";
import { engine } from "../data";
import { InstructionType, RenderInstruction } from "../render/instructions";
import { useMM } from "./use-mm";
import { measureText } from "../ui/utils/measure-text";
import { timer } from "../ui/utils/timer";
import { drawLine } from "../render/line";
import { drawText } from "../render/text";
import { drawCircle } from "../render/circle";
import { drawCurve } from "../render/curve";
import { drawShape } from "../render/shape";
import { drawBox } from "../render/box";

export function usePipeline(
  canvas: RefObject<HTMLCanvasElement>,
  flowKey: string,
  timings: boolean
) {
  const mm = useMM();

  useLayoutEffect(() => {
    timer("render", timings, () => {
      if (canvas?.current) {
        const ctx = canvas.current.getContext("2d", { alpha: false });
        const dpi = window.devicePixelRatio;

        if (ctx) {
          const [width, height, instructions] = engine.render(
            flowKey,
            mm,
            measureText
          );

          // setup canvas
          ctx.canvas.height = height * dpi;
          ctx.canvas.width = width * dpi;
          ctx.canvas.style.height = `${height}px`;
          ctx.canvas.style.width = `${width}px`;

          // clear canvas
          ctx.fillStyle = "#fff";
          ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

          // render instruction set
          instructions.forEach((instruction: RenderInstruction<any>) => {
            switch (instruction.type) {
              case InstructionType.Line:
                drawLine(ctx, instruction, dpi);
                break;
              case InstructionType.Text:
                drawText(ctx, instruction, dpi);
                break;
              case InstructionType.Circle:
                drawCircle(ctx, instruction, dpi);
                break;
              case InstructionType.Curve:
                drawCurve(ctx, instruction, dpi);
                break;
              case InstructionType.Shape:
                drawShape(ctx, instruction, dpi);
                break;
              case InstructionType.Box:
                drawBox(ctx, instruction, dpi);
                break;
              default:
                break;
            }
          });
        }
      }
    });
  });
}
