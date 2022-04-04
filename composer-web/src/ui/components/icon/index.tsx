import { MouseEvent, FC, CSSProperties } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  path: string;
  size: number;
  disabled?: boolean;
  toggled?: boolean;

  onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
}

/**
 * Icon which takes an svg path and renders.
 */
export const Icon: FC<Props> = ({ id, className, style, path, size, disabled, toggled, onClick }) => {
  if (onClick) {
    return (
      <button
        id={id}
        className={merge(
          "ui-icon",
          "ui-icon--hover",
          {
            "ui-icon--disabled": disabled,
            "ui-icon--toggleable": toggled !== undefined,
            "ui-icon--toggled": toggled,
          },
          className
        )}
        style={{
          width: size,
          height: size,
          minWidth: size,
          minHeight: size,
          ...style,
        }}
        onClick={onClick}
      >
        <svg className="ui-icon__svg" viewBox="0 0 24 24" style={{ width: size, height: size }}>
          <path className="ui-icon__svg-path" d={path} />
        </svg>
      </button>
    );
  } else {
    return (
      <div
        id={id}
        className={merge("ui-icon", className)}
        style={{
          width: size,
          height: size,
          minWidth: size,
          minHeight: size,
          ...style,
        }}
      >
        <svg className="ui-icon__svg" viewBox="0 0 24 24" style={{ width: size, height: size }}>
          <path className="ui-icon__svg-path" d={path} />
        </svg>
      </div>
    );
  }
};
