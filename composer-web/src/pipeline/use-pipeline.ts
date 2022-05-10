import { engine } from "../data";
import { useMM } from "./use-mm";
import { measureText } from "../ui/utils/measure-text";
import { timer } from "../ui/utils/timer";

interface Box {
  key: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

const BOX_SIZE = 400;
const canvases = (width: number, height: number) => {
  const output: Box[] = [];

  const columns = Math.floor(width / BOX_SIZE);
  const rows = Math.floor(height / BOX_SIZE);

  const columnWidth = width / columns;
  const rowHeight = height / rows;

  for (let y = 0; y < rows; y++) {
    for (let x = 0; x < columns; x++) {
      output.push({
        key: `${x}${y}`,
        x: x * columnWidth,
        y: y * rowHeight,
        width: columnWidth,
        height: rowHeight,
      });
    }
  }

  return output;
};

export function usePipeline(flowKey: string) {
  const mm = useMM();

  const [width, height, instructions] = timer("parse", true, () => {
    return engine.render(flowKey, mm, measureText);
  });

  return { canvases: canvases(width, height), instructions, width, height };
}
