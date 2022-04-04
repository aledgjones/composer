import { CSSProperties } from "react";
import * as React from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  open: boolean;
  transparent?: boolean;
  onClick?: () => void;
}

/**
 * Backdrop component used for overlays (dialogs, sheets etc.)
 */
export const Backdrop: React.FC<Props> = ({ id, className, open, transparent, onClick }) => {
  return (
    <div
      id={id}
      className={merge("ui-backdrop", className, {
        "ui-backdrop--hidden": !open,
        "ui-backdrop--transparent": transparent,
      })}
      onClick={onClick}
    />
  );
};
