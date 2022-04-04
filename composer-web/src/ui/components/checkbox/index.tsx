import { FC, CSSProperties } from "react";
import { mdiCheck } from "@mdi/js";
import merge from "classnames";
import { Icon } from "../icon";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  value: boolean;
  margin?: boolean;
  disabled?: boolean;
  onChange: (value: boolean) => void;
}

/**
 * Chackbox component for boolean values.
 */
export const Checkbox: FC<Props> = ({ id, className, style, children, value, onChange, disabled, margin }) => {
  return (
    <div
      id={id}
      className={merge(
        "ui-checkbox",
        {
          "ui-checkbox--active": value,
          "ui-checkbox--margin": margin,
          "ui-checkbox--disabled": disabled,
        },
        className
      )}
      style={style}
      onClick={() => onChange(!value)}
    >
      <div className="ui-checkbox__inner" style={{ marginRight: children ? 20 : 0 }}>
        {value && <Icon size={16} className="ui-checkbox__icon" path={mdiCheck} />}
      </div>
      {children && <div className="ui-checkbox__label">{children}</div>}
    </div>
  );
};
