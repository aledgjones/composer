import { FC, ReactNode } from "react";

import "./styles.css";

interface Props {
  href: string;
  target: string;
  children: ReactNode;
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
