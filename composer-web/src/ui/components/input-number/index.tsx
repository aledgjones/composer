import { FC, useCallback, useState, useEffect, KeyboardEvent } from "react";
import { mdiChevronUp, mdiChevronDown } from "@mdi/js";
import Big from "big.js";

import { error } from "../../utils/error";
import { Icon } from "../icon";
import { InputBase } from "../input-base";
import { NumberInputProps } from "../input-base/defs";

import "./styles.css";

export const InputNumber: FC<NumberInputProps> = ({
  value,
  required,
  step,
  precision,
  units,
  onChange,
  onBlur,
  ...props
}) => {
  const toValue = useCallback((value: string | undefined) => {
    if (value === undefined || value === "") {
      return undefined;
    } else {
      return parseFloat(value);
    }
  }, []);

  const toPrecision = useCallback(
    (value: number | string | undefined) => {
      if (value === undefined || value === "") {
        return "";
      } else {
        return new Big(value).toFixed(precision);
      }
    },
    [precision]
  );

  const [display, setDisplay] = useState<string>(toPrecision(value));

  useEffect(() => setDisplay(toPrecision(value)), [value, toPrecision]);

  const validate = useCallback(
    (display: string) => {
      if (!required && display === "") {
        return null;
      }

      if (required && display === "") {
        return error("@ui/input-required", "Required");
      }

      try {
        new Big(display); // big has stricter parsing so use for number validation
      } catch (e) {
        return error("@ui/input-invalid", "Invalid number");
      }

      return null;
    },
    [required]
  );

  const _onBlur = useCallback(() => {
    try {
      const val = toPrecision(display); // force precision and force valid numbers
      const parsed = toValue(val);
      setDisplay(val);
      onChange(parsed);
    } catch (e) {}

    if (onBlur) {
      onBlur();
    }
  }, [onBlur, toPrecision, toValue, display, value]);

  const onIncrease = useCallback(() => {
    if (value !== undefined) {
      const val = new Big(value).plus(step).toFixed(precision);
      const parsed = parseFloat(val);
      setDisplay(val);
      onChange(parsed);
    }
  }, [step, value, display, error, precision, onChange]);

  const onDecrease = useCallback(() => {
    if (value !== undefined) {
      const val = new Big(value).minus(step).toFixed(precision);
      const parsed = parseFloat(val);
      setDisplay(val);
      onChange(parsed);
    }
  }, [step, value, display, error, precision, onChange]);

  const onKeyDown = useCallback(
    (e: KeyboardEvent<HTMLInputElement>) => {
      switch (e.key) {
        case "ArrowUp":
          e.preventDefault();
          onIncrease();
          break;
        case "ArrowDown":
          e.preventDefault();
          onDecrease();
          break;
        default:
          break;
      }
    },
    [onIncrease, onDecrease]
  );

  return (
    <InputBase
      display={display}
      required={required}
      spellcheck={false}
      validate={validate}
      onBlur={_onBlur}
      onChange={setDisplay}
      onKeyDown={onKeyDown}
      {...props}
    >
      {units && <p className="ui-input-number__units">{units}</p>}
      <div className="ui-input-number__controls">
        <Icon path={mdiChevronUp} size={18} onClick={onIncrease} />
        <Icon path={mdiChevronDown} size={18} onClick={onDecrease} />
      </div>
    </InputBase>
  );
};
