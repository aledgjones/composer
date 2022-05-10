import { engine } from "../data";
import { useMM } from "./use-mm";
import { measureText } from "../ui/utils/measure-text";
import { timer } from "../ui/utils/timer";
import { calculateCanvasTiles } from "./calculate-canvas-tiles";

export function usePipeline(flowKey: string) {
  const mm = useMM();

  const [width, height, instructions] = timer("parse", true, () => {
    return engine.render(flowKey, mm, measureText);
  });

  const tiles = calculateCanvasTiles(width, height);

  return { tiles, instructions, width, height };
}
