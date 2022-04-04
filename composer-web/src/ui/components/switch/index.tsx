import { CSSProperties, FC } from "react";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  value: boolean;
  disabled?: boolean;

  onChange?: (val: boolean) => void;
}

/**
 * Switch component.
 */
export const Switch: FC<Props> = ({ id, className, style, value, disabled, onChange }) => {
  return (
    <div
      id={id}
      className={merge("ui-switch", { "ui-switch--active": value, "ui-switch--disabled": disabled }, className)}
      style={style}
      onClick={() => onChange && onChange(!value)}
    >
      <div className="ui-switch__track" />
      <div className="ui-switch__button" />
    </div>
  );
};
