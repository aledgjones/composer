import { CSSProperties, FC, ReactNode } from "react";
import { useInView } from "react-intersection-observer";

interface Props {
  style?: CSSProperties;
  className?: string;
  children: ReactNode;
}

export const Offscreen: FC<Props> = ({ style, className, children }) => {
  const { inView, ref } = useInView();

  return (
    <div className={className} style={style} ref={ref}>
      {inView && children}
    </div>
  );
};
