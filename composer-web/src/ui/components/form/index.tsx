import { FC, CSSProperties } from "react";
import merge from "classnames";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
}

/**
 * Form element with onSubmit prevented by default.
 */
export const Form: FC<Props> = ({ id, className, style, children }) => {
  return (
    <form id={id} className={merge("ui-form", className)} style={style} onSubmit={(e) => e.preventDefault()}>
      {children}
    </form>
  );
};
