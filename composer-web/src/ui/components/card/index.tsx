import { FC, CSSProperties, ReactNode } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  margin?: boolean;
  animate?: boolean;

  children: ReactNode;
}

/**
 * Generic card component.
 */
export const Card: FC<Props> = ({
  id,
  className,
  style,
  children,
  margin,
  animate,
}) => {
  return (
    <div
      id={id}
      className={merge(
        "ui-card",
        { "ui-card--margin": margin, "ui-card--animate": animate },
        className
      )}
      style={style}
    >
      {children}
    </div>
  );
};
