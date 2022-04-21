import { FC, ReactNode } from "react";

interface Props {
  value: any;
  displayAs: string;

  children?: ReactNode;
}

/**
 * Option to be used with Select element.
 */
export const Option: FC<Props> = ({ children }) => {
  return <>{children}</>;
};
