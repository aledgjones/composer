import { RefObject, useLayoutEffect } from "react";
import { engine } from "../data";
import { drawBox } from "../render/draw-box";
import { drawCircle } from "../render/draw-circle";
import { drawCurve } from "../render/draw-curve";
import { drawLine } from "../render/draw-line";
import { drawShape } from "../render/draw-shape";
import { drawText } from "../render/draw-text";
import { InstructionType, RenderInstruction } from "../render/instructions";
import { useMM } from "./use-mm";
import { measureText } from "../ui/utils/measure-text";
import { timer } from "../ui/utils/timer";

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

        const setup = (height: number, width: number) => {
          if (ctx) {
            ctx.canvas.height = height * dpi;
            ctx.canvas.width = width * dpi;
            ctx.canvas.style.height = `${height}px`;
            ctx.canvas.style.width = `${width}px`;

            // clear
            ctx.fillStyle = "#fff";
            ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
          }
        };

        const render = (instruction: RenderInstruction<any>) => {
          if (ctx) {
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
          }
        };

        engine.render(flowKey, mm, setup, render, measureText);
      }
    });
  });
}
