import { FC } from "react";
import { engine } from "../../data";
import merge from "classnames";
import { usePipeline } from "../../pipeline/use-pipeline";
import { Cell } from "./cell";

import "./styles.css";

interface Props {
  className?: string;
  flowKey: string;
}

export const Renderer: FC<Props> = ({ className, flowKey }) => {
  const { width, height, tiles, instructions } = usePipeline(flowKey);

  return (
    <div className={merge("renderer", className)}>
      <div className="renderer__container" style={{ width, height }}>
        <p className="renderer__flow-name">
          {engine.get_flow_title(flowKey) || "Untitled Flow"}
        </p>
        {tiles.map(({ key, x, y, width, height }) => {
          return (
            <Cell
              key={key}
              instructions={instructions}
              x={x}
              y={y}
              width={width}
              height={height}
            />
          );
        })}
      </div>
    </div>
  );
};
