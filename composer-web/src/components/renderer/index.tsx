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
  const { width, height, canvases, instructions } = usePipeline(flowKey);

  return (
    <div className={merge("renderer", className)}>
      <div className="renderer__container" style={{ width, height }}>
        <p className="renderer__flow-name">
          {engine.get_flow_title(flowKey) || "Untitled Flow"}
        </p>
        {canvases.map((config) => {
          return (
            <Cell
              key={config.key}
              instructions={instructions}
              x={config.x}
              y={config.y}
              width={config.width}
              height={config.height}
            />
          );
        })}
      </div>
    </div>
  );
};
