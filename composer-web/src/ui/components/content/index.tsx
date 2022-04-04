import { FC, CSSProperties } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
}

/**
 * Content component with default padding.
 */
export const Content: FC<Props> = ({ id, className, style, children }) => {
  return (
    <div id={id} className={merge("ui-content", className)} style={style}>
      {children}
    </div>
  );
};
