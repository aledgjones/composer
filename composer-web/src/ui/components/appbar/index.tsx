import { SuperFC } from "../../generic";
import merge from "classnames";

import "./styles.css";

interface Props {
  shadow?: boolean;
}
/**
 * A basic top of screen app bar with dynamic shadow.
 */
export const Appbar: SuperFC<HTMLDivElement, Props> = ({ className, shadow, children, ...props }) => {
  return (
    <header className={merge("ui-appbar", { "ui-appbar--shadow": shadow }, className)} {...props}>
      <div className="ui-appbar__content">{children}</div>
    </header>
  );
};
