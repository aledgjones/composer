import { CSSProperties, FC, ReactNode } from "react";

import { useDelayBoolean } from "../../hooks/use-delay-boolean";
import merge from "classnames";
import { Portal } from "../portal";
import { Backdrop } from "../backdrop";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  width: number;

  open: boolean;
  onClose: () => void;
  children: () => ReactNode;
}

/**
 * Bottom Sheet component for displaying contexual actions.
 */
export const BottomSheet: FC<Props> = ({ id, className, style, width, open, onClose, children }) => {
  const render = useDelayBoolean(open, 500);

  return (
    <Portal>
      <Backdrop open={open} onClick={onClose} />
      <div
        id={id}
        className={merge("ui-bottom-sheet", className)}
        style={{
          maxWidth: width,
          maxHeight: "100vh", // TO DO expanded state uses this
          overflow: "auto",
          transform: open ? "translate(-50%, -100%)" : "translate(-50%, 0)",
          ...style,
        }}
      >
        {render && children()}
      </div>
    </Portal>
  );
};
