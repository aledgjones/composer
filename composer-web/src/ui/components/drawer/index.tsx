import { CSSProperties, FC, ReactNode } from "react";

import merge from "classnames";
import { useDelayBoolean } from "../../hooks/use-delay-boolean";
import { Portal } from "../portal";
import { Backdrop } from "../backdrop";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  width: number;
  open: boolean;

  // events may happen that closes/opens from within component
  onClose: () => void;
  children: () => ReactNode;
}

/**
 * App drawer component.
 */
export const Drawer: FC<Props> = ({ id, className, style, children, open, width, onClose }) => {
  const render = useDelayBoolean(open, 500);

  return (
    <Portal>
      <Backdrop open={open} onClick={onClose} />
      <div
        id={id}
        className={merge("ui-drawer", className)}
        style={{
          maxWidth: width,
          transform: `translate3d(${open ? 0 : "-100%"}, 0, 0)`,
          ...style,
        }}
      >
        {render && children()}
      </div>
    </Portal>
  );
};
