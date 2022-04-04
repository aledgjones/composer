import { FC } from "react";

import "./styles.css";

interface Props {
  href: string;
  target: string;
}
export const Link: FC<Props> = ({ href, target, children }) => {
  return (
    <a
      className="ui-link"
      href={href}
      rel="noreferrer noopener"
      target={target}
    >
      {children}
    </a>
  );
};
