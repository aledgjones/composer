import { FC, useState, useEffect, useCallback } from "react";

import merge from "classnames";
import { Button } from "../button";
import { ToastInstance } from "./defs";

import "./styles.css";

interface Props {
  color: string;
  toast: ToastInstance;
  onDestroy: (key: string) => void;
}

/**
 * Internal component used by the Toast component.
 */
export const ToastEntry: FC<Props> = ({ color, toast, onDestroy }) => {
  const [hidden, setHidden] = useState(false);
  const [selfCombustTimeout, setSelfCombustTimeout] = useState<any>();

  useEffect(() => {
    setHidden(false);
    const timeout = setTimeout(() => {
      if (toast.onTimeout) {
        toast.onTimeout();
      }
      setHidden(true);
      setTimeout(() => onDestroy(toast.key), 500);
    }, toast.duration);
    setSelfCombustTimeout(timeout);
  }, [toast, onDestroy]);

  const onClick = useCallback(() => {
    clearTimeout(selfCombustTimeout);
    if (toast.onClick) {
      toast.onClick();
    }
    setHidden(true);
    setTimeout(() => onDestroy(toast.key), 500);
  }, [toast, selfCombustTimeout, onDestroy]);

  return (
    <div key={toast.key} className={merge("ui-toast", { "ui-toast--hide": hidden })}>
      <span className="ui-toast__text">{toast.text}</span>
      {toast.onClick && (
        <Button className="ui-toast__button" compact outline onClick={onClick}>
          {toast.button || "Cancel"}
        </Button>
      )}
    </div>
  );
};
