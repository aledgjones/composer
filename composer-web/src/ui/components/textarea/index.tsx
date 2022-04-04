import { CSSProperties, FC, useState, useCallback } from "react";

import merge from "classnames";

import "../input-base/styles.css";
import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  value: any;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  spellcheck?: boolean;
  margin?: boolean;

  onChange: (value: any) => void;
}

/**
 * Auto expanding textarea component.
 */
export const Textarea: FC<Props> = ({
  id,
  className,
  style,
  value,
  placeholder,
  required,
  disabled,
  spellcheck,
  margin,
  onChange,
}) => {
  const validate = useCallback(
    (value: string) => {
      if (required && value === "") {
        return "Required";
      } else {
        return null;
      }
    },
    [required]
  );

  const [focus, setFocus] = useState<boolean>(false);
  const [touched, setTouched] = useState(false);
  const error = touched ? validate(value) : null;

  return (
    <div
      id={id}
      className={merge(
        "ui-textarea",
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
      <div className="ui-input__container ui-textarea__container" style={style}>
        <textarea
          className="ui-input__display ui-textarea__display"
          value={value}
          spellCheck={spellcheck}
          onChange={(e) => onChange(e.target.value)}
          onFocus={() => setFocus(true)}
          onBlur={() => {
            setFocus(false);
            setTouched(true);
          }}
        />
        <div className="ui-input__display ui-textarea__slave">
          {value}
          {"\n"}
        </div>
      </div>
      {error && <p className="ui-input__error-text">{error}</p>}
    </div>
  );
};
