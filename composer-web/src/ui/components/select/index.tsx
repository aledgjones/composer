import {
  useState,
  useMemo,
  CSSProperties,
  FC,
  useRef,
  useEffect,
  Children,
  ReactNode,
} from "react";
import { mdiChevronDown } from "@mdi/js";

import merge from "classnames";
import { Icon } from "../icon";
import { Card } from "../card";
import { useDelayBoolean } from "../../hooks/use-delay-boolean";
import { noop } from "../../utils/noop";

import "../input-base/styles.css";
import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  value: any;
  margin?: boolean;
  disabled?: boolean;
  direction?: "up" | "down";

  onChange: (value: any) => void;

  children: ReactNode;
}

/**
 * Select component to be used with the Option component.
 */
export const Select: FC<Props> = ({
  id,
  className,
  style,
  value,
  margin,
  children,
  onChange,
  disabled,
  direction,
}) => {
  const [focus, setFocus] = useState<boolean>(false);
  const element = useRef<HTMLDivElement>(null);

  const open = useDelayBoolean(focus, 400);

  const display = useMemo(() => {
    let _display = "";
    Children.forEach(children, (child: any) => {
      if (child && child.props.value === value) {
        _display = child.props.displayAs;
      }
    });
    return _display;
  }, [value, children]);

  // auto close
  useEffect(() => {
    const cb = (e: any) => {
      if (!element.current || !element.current.contains(e.target)) {
        setFocus(false);
      } else {
        setFocus((o) => !o);
      }
    };
    document.addEventListener("click", cb);
    return () => document.removeEventListener("click", cb);
  }, [element]);

  return (
    <div
      id={id}
      className={merge(
        "ui-select",
        "ui-input",
        {
          "ui-input--disabled": disabled,
          "ui-input--focus": focus,
          "ui-input--margin": margin,
        },
        className
      )}
      style={style}
      ref={element}
    >
      <div className="ui-select__container ui-input__container" style={style}>
        <p className="ui-input__display">{display}</p>
        <Icon
          className="ui-select__icon"
          style={{ transform: focus ? "rotateZ(180deg)" : undefined }}
          size={24}
          path={mdiChevronDown}
          onClick={noop}
        />
      </div>

      <Card
        className={merge("ui-select__card", {
          "ui-select__card--open": focus,
          "ui-select__card--up": direction === "up",
        })}
      >
        {open &&
          Children.map(children, (child: any) => {
            if (child) {
              return (
                <button
                  key={child.props.value}
                  className="ui-select__item"
                  onClick={() => {
                    onChange(child.props.value);
                  }}
                >
                  {child}
                </button>
              );
            } else {
              return null;
            }
          })}
      </Card>
    </div>
  );
};
