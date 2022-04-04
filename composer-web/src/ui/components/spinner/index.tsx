import { CSSProperties, FC, useMemo } from "react";
import Big from "big.js";

import merge from "classnames";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  size: number;
  color: string;
  percent?: number;
}

/**
 * A spinner which can be indeterminate or determinate to denote working states or progress.
 */
export const Spinner: FC<Props> = ({ id, className, style, size, color, percent }) => {
  const [dashoffset, dasharray] = useMemo(() => {
    if (percent !== undefined) {
      const r = 20;
      const c = new Big(2 * Math.PI * r); // circumference @ r=20;
      const _value = new Big(percent).div(100).times(c);
      return [c.minus(_value).toFixed(2), c.toFixed(2)];
    } else {
      return [undefined, undefined];
    }
  }, [percent]);

  const animate = percent === undefined;

  return (
    <svg
      id={id}
      className={merge("ui-spinner", { "ui-spinner--animate": animate }, className)}
      style={{
        height: size,
        width: size,
        ...style,
      }}
      viewBox="25 25 50 50"
    >
      {!animate && <circle className="ui-spinner__circle" cx="50" cy="50" r="20" stroke="rgb(175, 175, 175)" />}
      <circle
        className="ui-spinner__circle"
        cx="50"
        cy="50"
        r="20"
        stroke={color}
        strokeDasharray={dasharray}
        strokeDashoffset={dashoffset}
      />
    </svg>
  );
};
