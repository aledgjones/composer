import { FC } from "react";
import { Idx, TickList } from "../ticks/defs";

import "./styles.css";

interface Props {
  ticks: TickList;
  zoom: number;
}

export const PlayHead: FC<Props> = ({ ticks, zoom }) => {
  // const tick = useTick();
  const tick = 0;

  return (
    <div
      className="play-head"
      style={{
        transform: `translate3d(${
          ticks.list[tick]
            ? ticks.list[tick][Idx.X] * zoom - 1
            : ticks.width * zoom - 1
        }px, 0, 0)`,
      }}
    />
  );
};
