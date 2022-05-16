import { FC, ReactNode } from "react";

import "./styles.css";

interface Props {
  children: ReactNode;
}

export const BottomBar: FC<Props> = ({ children }) => {
  return <div className="bottom-bar">{children}</div>;
};
