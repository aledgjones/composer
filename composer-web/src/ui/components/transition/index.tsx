import { FC, CSSProperties } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  type: "swipeUp" | "fadeIn";
}

/**
 * Basic animation in for elements.
 */
export const Transition: FC<Props> = ({ id, className, style, type, children }) => {
  return (
    <div
      id={id}
      className={merge("ui-transition", className, {
        "ui-transition--swipe-up": type === "swipeUp",
        "ui-transition--fade-in": type === "fadeIn",
      })}
      style={style}
    >
      {children}
    </div>
  );
};
