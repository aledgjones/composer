import { FC, CSSProperties, ReactNode } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
  children: ReactNode;
}

export const Label: FC<Props> = ({ id, className, style, children }) => {
  return (
    <div id={id} className={merge("ui-label", className)} style={style}>
      {children}
    </div>
  );
};
