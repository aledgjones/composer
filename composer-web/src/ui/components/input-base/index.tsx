import { FC, useState, useCallback, ChangeEvent } from "react";

import merge from "classnames";
import { InternalInputBaseProps } from "./defs";

import "./styles.css";

export const InputBase: FC<InternalInputBaseProps> = ({
  id,
  className,
  style,
  type,
  display,
  margin,
  disabled,
  spellcheck,
  validate,
  onChange,
  onBlur,
  onFocus,
  onKeyDown,
  children,
}) => {
  const [focus, setFocus] = useState<boolean>(false);
  const [touched, setTouched] = useState(false);

  const error = touched ? validate(display) : null;

  const _onChange = useCallback((e: ChangeEvent<HTMLInputElement>) => onChange(e.target.value), [onChange]);

  const _onFocus = useCallback(() => {
    if (onFocus) {
      onFocus();
    }
    setFocus(true);
  }, [onFocus]);

  const _onBlur = useCallback(() => {
    if (onBlur) {
      onBlur();
    }
    setFocus(false);
    setTouched(true);
  }, [onBlur]);

  return (
    <div
      id={id}
      className={merge(
        "ui-input",
        {
          "ui-input--error": !!error,
          "ui-input--focus": focus,
          "ui-input--disabled": disabled,
          "ui-input--margin": margin,
        },
        className
      )}
    >
      <div className="ui-input__container" style={style}>
        <input
          className="ui-input__display"
          type={type === "password" ? "password" : "text"}
          value={display}
          spellCheck={spellcheck}
          onChange={_onChange}
          onFocus={_onFocus}
          onBlur={_onBlur}
          onKeyDown={onKeyDown}
        />
        {children}
      </div>
      {error && !disabled && <p className="ui-input__error-text">{error.message}</p>}
    </div>
  );
};
