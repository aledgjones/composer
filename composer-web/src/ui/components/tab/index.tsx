import { FC, useRef, useCallback, useEffect, ReactNode } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  value: any;
  children: ReactNode;
}

/**
 * Tab component to be used inside the Tabs component;
 */
export const Tab: FC<Props> = ({ children }) => {
  return <>{children}</>;
};

interface PropsExtended extends Props {
  selected: boolean;
  onChange: (value: any) => void;
  setBar: (value: { left: number; width: number }) => void;
}

/**
 * Tab component to be used inside the Tabs component;
 */
export const TabExtended: FC<PropsExtended> = ({
  children,
  value,
  selected,
  onChange,
  setBar,
}) => {
  const $element = useRef<HTMLButtonElement>(null);

  const onClick = useCallback(() => {
    if (onChange) {
      onChange(value);
    }
  }, [value, onChange]);

  useEffect(() => {
    if (selected && $element.current) {
      setBar({
        left: $element.current.offsetLeft,
        width: $element.current.offsetWidth,
      });
    }
  }, [selected, setBar, $element]);

  return (
    <button
      ref={$element}
      className={merge("ui-tab", { "ui-tab--selected": selected })}
      onClick={onClick}
    >
      {children}
    </button>
  );
};
