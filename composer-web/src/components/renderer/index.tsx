import { FC } from "react";
import { engine } from "../../data";
import merge from "classnames";
import { usePipeline } from "../../pipeline/use-pipeline";
import { Offscreen } from "../../ui/components/offscreen";
import { Canvas } from "./canvas";

import "./styles.css";

interface Props {
  className?: string;
  flowKey: string;
  onSelect?: (x: number, y: number) => void;
}

export const Renderer: FC<Props> = ({ className, flowKey, onSelect }) => {
  const { width, height, tiles, instructions } = usePipeline(flowKey);

  return (
    <Offscreen
      className={merge("renderer", className)}
      style={{
        width,
        height,
      }}
    >
      <p className="renderer__flow-name">
        {engine.get_flow_title(flowKey) || "Untitled Flow"}
      </p>
      {tiles.map(({ key, x, y, width, height }) => {
        return (
          <Offscreen
            key={key}
            style={{
              position: "absolute",
              top: y,
              left: x,
              width,
              height,
            }}
          >
            <Canvas
              onSelect={onSelect}
              instructions={instructions}
              x={x}
              y={y}
              width={width}
              height={height}
            />
          </Offscreen>
        );
      })}
    </Offscreen>
  );
};
