import { MutableRefObject, useLayoutEffect } from "react";
import { engine } from "../engine";
import { drawBox } from "../render/draw-box";
import { drawCircle } from "../render/draw-circle";
import { drawCurve } from "../render/draw-curve";
import { drawLine } from "../render/draw-line";
import { drawShape } from "../render/draw-shape";
import { drawText } from "../render/draw-text";
import { InstructionType, RenderInstruction } from "../render/instructions";
import { useMM } from "./use-mm";

export function usePipeline(
  canvas: MutableRefObject<HTMLCanvasElement>,
  flowKey: string,
  timings: boolean
) {
  // ensure we render when canvas is ready
  const mm = useMM();

  useLayoutEffect(() => {
    if (timings) {
      performance.mark("start-render");
    }

    if (canvas.current) {
      const ctx = canvas.current.getContext("2d", { alpha: false });
      const dpi = window.devicePixelRatio;

      const setup = (height: number, width: number) => {
        ctx.canvas.height = height * dpi;
        ctx.canvas.width = width * dpi;
        ctx.canvas.style.height = `${height}px`;
        ctx.canvas.style.width = `${width}px`;

        // clear
        ctx.fillStyle = "#fff";
        ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
      };

      const render = (instruction: RenderInstruction<any>) => {
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
      };

      engine.render(flowKey, mm, setup, render);
    }

    if (timings) {
      performance.measure("render", "start-render");
      const entries = performance.getEntriesByType("measure");
      for (let i = 0; i < entries.length; i++) {
        const entry = entries[i];
        console.log(
          `${entry.name}: %c${entry.duration}`,
          (entry.duration < 1000 / 60 && "color: green") ||
            (entry.duration < 1000 / 30 && "color: orange") ||
            "color: red"
        );
      }
      performance.clearMarks();
      performance.clearMeasures();
    }
  }, [flowKey, canvas, timings]);
}
