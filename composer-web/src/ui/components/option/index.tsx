import { FC } from "react";

interface Props {
  value: any;
  displayAs: string;
}

/**
 * Option to be used with Select element.
 */
export const Option: FC<Props> = ({ children }) => {
  return <>{children}</>;
};
