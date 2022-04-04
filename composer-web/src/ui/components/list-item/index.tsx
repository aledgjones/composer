import { FC, CSSProperties } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  disabled?: boolean;

  onClick?: () => void;
}

/**
 * List Item with default hover styles if onClick present.
 */
export const ListItem: FC<Props> = ({ id, className, style, disabled, onClick, children }) => {
  return (
    <div
      id={id}
      className={merge(
        "ui-list-item",
        {
          "ui-list-item--hover": !!onClick,
          "ui-list-item--disabled": disabled,
        },
        className
      )}
      style={style}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
