import { Tick } from "./defs";

export const getTickHeight = (
  tick: Tick,
  isTrack: boolean,
  height: number,
  zoom: number
) => {
  if (tick.first) {
    return height;
  } else if (tick.boundry) {
    return isTrack ? height : height / 2;
  } else if (tick.beat) {
    return isTrack ? height : height / 3;
  } else if (tick.sub_beat) {
    if (zoom >= 0.5) {
      return isTrack ? height : height / 6;
    } else {
      return 0;
    }
  } else {
    return undefined;
  }
};

export const getTickColor = (tick: Tick, isTrack: boolean) => {
  if (!isTrack) {
    return "var(--background-1000)";
  } else {
    if (tick.first) {
      return "var(--background-200)";
    } else if (tick.beat) {
      return "var(--background-500)";
    } else if (tick.sub_beat) {
      return "var(--background-600)";
    } else {
      return undefined;
    }
  }
};
