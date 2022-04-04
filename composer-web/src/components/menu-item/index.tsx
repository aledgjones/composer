import { FC } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  selected?: boolean;
  onClick?: () => void;
}

export const MenuItem: FC<Props> = ({ selected, onClick, children }) => {
  return (
    <div
      className={merge("menu-item", {
        "menu-item--clickable": !!onClick,
        "menu-item--selected": selected,
      })}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
