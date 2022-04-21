import { FC, ReactNode } from "react";
import { DragScroll } from "../../ui/components/drag-scroll";
import merge from "classnames";

import "./styles.css";

interface Props {
  className?: string;
  children: ReactNode;
}

export const RenderRegion: FC<Props> = ({ children, className }) => {
  return (
    <DragScroll
      ignore="no-scroll"
      x
      y
      className={merge("render-region", className)}
    >
      <div className="render-region__padding">{children}</div>
    </DragScroll>
  );
};
