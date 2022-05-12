import { FC, CSSProperties, ReactNode } from "react";
import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
  children: ReactNode;

  onClick?: () => void;
}

export const List: FC<Props> = ({
  id,
  className,
  style,
  onClick,
  children,
}) => {
  return (
    <div
      id={id}
      className={merge("ui-list", className)}
      style={style}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
