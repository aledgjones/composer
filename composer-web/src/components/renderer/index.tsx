import { FC, useRef } from "react";
import { engine } from "../../data";
import merge from "classnames";

import { usePipeline } from "../../pipeline/use-pipeline";

import "./styles.css";

interface Props {
  className?: string;
  flowKey: string;
}

export const Renderer: FC<Props> = ({ className, flowKey }) => {
  const canvas = useRef<HTMLCanvasElement>(null);

  usePipeline(canvas, flowKey, true);

  return (
    <div className={merge("renderer", className)}>
      <div className="renderer__container">
        <p className="renderer__flow-name">
          {engine.get_flow_title(flowKey) || "Untitled Flow"}
        </p>
        <canvas ref={canvas} className="renderer__canvas" />
      </div>
    </div>
  );
};
