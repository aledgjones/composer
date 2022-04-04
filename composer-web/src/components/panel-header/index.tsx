import { FC } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  className?: string;
}

export const PanelHeader: FC<Props> = ({ children, className }) => {
  return <div className={merge("panel-header", className)}>{children}</div>;
};
