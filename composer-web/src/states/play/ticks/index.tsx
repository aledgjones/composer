import { Fragment, FC } from "react";
import { TickList, Tick } from "../../../store/score-flow/defs";
import merge from "classnames";

function getTickHeight(tick: Tick, isTrack: boolean, height: number, zoom: number) {
  if (tick.isFirstBeat) {
    return height;
  } else if (tick.isGroupingBoundry) {
    return isTrack ? height : height / 2;
  } else if (tick.isBeat) {
    return isTrack ? height : height / 3;
  } else if (tick.isQuaverBeat) {
    if (zoom >= 0.5) {
      return isTrack ? height : height / 6;
    } else {
      return 0;
    }
  } else {
    return undefined;
  }
}

function getTickColor(tick: Tick, isTrack: boolean) {
  if (!isTrack) {
    return "var(--background-1000)";
  } else {
    if (tick.isFirstBeat) {
      return "var(--background-200)";
    } else if (tick.isBeat) {
      return "var(--background-500)";
    } else if (tick.isQuaverBeat) {
      return "var(--background-600)";
    } else {
      return undefined;
    }
  }
}

interface Props {
  className?: string;
  ticks: TickList;
  height: number;
  isTrack: boolean;
  zoom: number;
}

export const Ticks: FC<Props> = ({ className, ticks, height, isTrack, zoom }) => {
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
          if (tick.isFirstBeat) {
            bar++;
          }
          return (
            <Fragment key={i}>
              <line x1={tick.x * zoom} y1="0" x2={tick.x * zoom} y2={tickHeight} strokeWidth="1" stroke={tickColor} />
              {!isTrack && tick.isFirstBeat && (
                <text x={tick.x * zoom + 5} y={12} fill="var(--background-1000)" fontSize="10" fontFamily="Roboto">
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
