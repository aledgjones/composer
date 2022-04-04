import { CSSProperties, FC } from "react";

import merge from "classnames";
import { useDelayBoolean } from "../../hooks/use-delay-boolean";
import { Portal } from "../portal";
import { Backdrop } from "../backdrop";
import { Card } from "../card";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
  width?: number;
  open: boolean;
}

/**
 * Dialog component for displaying related but long form actions/information.
 */
export function Dialog<T>(Content: FC<T>): FC<T & Props> {
  return ({ width, open, id, className, style, ...props }) => {
    const render = useDelayBoolean(open, 500);

    if (render) {
      return (
        <Portal>
          <Backdrop className="ui-dialog__backdrop" open={open} />
          <div className={merge("ui-dialog", { "ui-dialog--hidden": !open })}>
            <Card id={id} className={merge("ui-dialog__card", className)} style={{ maxWidth: width, ...style }}>
              <Content {...(props as T)} />
            </Card>
          </div>
        </Portal>
      );
    } else {
      return null;
    }
  };
}
