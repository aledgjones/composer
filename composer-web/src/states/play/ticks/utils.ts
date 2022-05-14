import { Idx, Tick } from "./defs";

export const getTickHeight = (
  tick: Tick,
  isTrack: boolean,
  height: number,
  zoom: number
) => {
  if (tick[Idx.First]) {
    return height;
  } else if (tick[Idx.Boundary]) {
    return isTrack ? height : height / 2;
  } else if (tick[Idx.Beat]) {
    return isTrack ? height : height / 3;
  } else if (tick[Idx.SubBeat]) {
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
    if (tick[Idx.First]) {
      return "var(--background-200)";
    } else if (tick[Idx.Beat]) {
      return "var(--background-500)";
    } else if (tick[Idx.SubBeat]) {
      return "var(--background-600)";
    } else {
      return undefined;
    }
  }
};
