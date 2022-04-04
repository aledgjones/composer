import { CSSProperties, FC } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
  compact?: boolean;
}

/**
 * Simple devider component.
 */
export const Divider: FC<Props> = ({ id, className, style, children, compact }) => {
  return (
    <div id={id} className={merge("ui-divider", { "ui-divider--compact": compact }, className)} style={style}>
      {children}
    </div>
  );
};
