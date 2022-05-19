import { FC } from "react";
import { useTick } from "../../../data/utils";
import { Idx, TickList } from "../ticks/defs";

import "./styles.css";

interface Props {
  ticks: TickList;
  zoom: number;
}

export const PlayHead: FC<Props> = ({ ticks, zoom }) => {
  const tick = useTick();
  const x = ticks.list[tick][Idx.X] * zoom - 1;

  return (
    <div
      className="play-head"
      style={{
        transform: `translate3d(${x}px, 0, 0)`,
      }}
    />
  );
};
