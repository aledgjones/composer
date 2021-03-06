import { Fragment, FC } from "react";
import merge from "classnames";
import { Idx, TickList } from "./defs";
import { getTickColor, getTickHeight } from "./utils";

interface Props {
  className?: string;
  ticks: TickList;
  height: number;
  isTrack: boolean;
  zoom: number;
}

export const Ticks: FC<Props> = ({
  className,
  ticks,
  height,
  isTrack,
  zoom,
}) => {
  let bar = 0;
  return (
    <svg
      viewBox={`0 0 ${ticks.width * zoom} ${height}`}
      className={merge("ticks", className)}
      style={{ width: ticks.width * zoom, height }}
    >
      {ticks.list.map((tick, i) => {
        const tickHeight = getTickHeight(tick, isTrack, height, zoom);
        if (tickHeight) {
          const tickColor = getTickColor(tick, isTrack);
          if (tick[Idx.First]) {
            bar++;
          }
          return (
            <Fragment key={i}>
              <line
                x1={tick[Idx.X] * zoom}
                y1="0"
                x2={tick[Idx.X] * zoom}
                y2={tickHeight}
                strokeWidth="1"
                stroke={tickColor}
              />
              {!isTrack && tick[Idx.First] && (
                <text
                  x={tick[Idx.X] * zoom + 5}
                  y={12}
                  fill="var(--background-1000)"
                  fontSize="10"
                  fontFamily="Roboto"
                >
                  {bar}
                </text>
              )}
            </Fragment>
          );
        } else {
          return null;
        }
      })}
    </svg>
  );
};
