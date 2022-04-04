import { CSSProperties, MouseEvent, FC } from "react";

import { useForeground } from "../../hooks/use-foreground";
import merge from "classnames";
import { Icon } from "../icon";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  path: string;
  color: string;
  text: string;
  compact?: boolean;
  hidden?: boolean;

  onClick?: (e: MouseEvent<HTMLDivElement>) => void;
}

/**
 * Floating action button (FAB)
 */
export const Fab: FC<Props> = ({ id, className, style, path, color, onClick, compact, hidden, text }) => {
  const fg = useForeground(color);

  return (
    <div
      id={id}
      className={merge("ui-fab", { "ui-fab--hidden": hidden }, className)}
      style={{ backgroundColor: color, color: fg, ...style }}
      onClick={onClick}
    >
      <Icon size={24} path={path} />
      <div className={merge("ui-fab__text", { "ui-fab__text--hidden": compact })}>{text}</div>
    </div>
  );
};
