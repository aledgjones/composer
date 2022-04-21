import { SLOT_HEIGHT } from "./const";
import { TickList } from "../../store/score-flow/defs";

export function getStartOfTone(
  x: number,
  initX: number,
  ticks: TickList,
  snap: number,
  zoom: number,
  start: number,
  duration: number,
  fixedStart: boolean,
  fixedDuration: boolean
) {
  if (fixedStart) {
    return start;
  } else {
    const s = start + (getTickFromXPosition(x, ticks, snap, zoom) - getTickFromXPosition(initX, ticks, snap, zoom));
    if (fixedDuration) {
      if (s < 0) {
        return 0;
      } else if (s + duration > ticks.list.length) {
        // avoid overshooting the track
        return ticks.list.length - duration;
      } else {
        return s;
      }
    } else {
      const max = start + duration;
      return s < max ? s : max;
    }
  }
}

export function getDurationOfTone(
  x: number,
  ticks: TickList,
  snap: number,
  zoom: number,
  start: number,
  duration: number,
  fixedStart: boolean,
  fixedDuration: boolean
) {
  if (fixedDuration) {
    return duration;
  } else {
    const d = fixedStart
      ? getTickFromXPosition(x, ticks, snap, zoom) - start
      : duration - (getTickFromXPosition(x, ticks, snap, zoom) - start);

    if (d < 0) {
      return 0;
    } else if (start + d === ticks.list.length) {
      return ticks.list.length - start;
    } else {
      return d;
    }
  }
}

export function getPitchFromYPosition(y: number, highestPitch: number, slots: number) {
  const lowestPitch = highestPitch - (slots - 1);
  const slot = Math.floor(y / SLOT_HEIGHT);
  const pitch = highestPitch - slot;

  // avoid dragging beyond the bounds of the track.
  if (pitch > highestPitch) {
    return highestPitch;
  } else if (pitch < lowestPitch) {
    return lowestPitch;
  } else {
    return pitch;
  }
}

export function getTickFromXPosition(x: number, ticks: TickList, snap: number, zoom: number, round?: "up" | "down") {
  for (let i = 0; i < ticks.list.length; i++) {
    const tick = ticks.list[i];
    if (tick.x * zoom > x) {
      // we have overshot, it is in the previous tick
      const index = i - 1;
      const lowerSnapTick = index - (index % snap);
      const higherSnapTick = lowerSnapTick + snap;

      const highestX = ticks.list[higherSnapTick] ? ticks.list[higherSnapTick].x * zoom : ticks.width * zoom;
      const middleOfSnap = ticks.list[lowerSnapTick].x * zoom + (highestX - ticks.list[lowerSnapTick].x * zoom) / 2;

      if (round === "down") {
        return lowerSnapTick;
      }

      if (round === "up") {
        return higherSnapTick;
      }

      if (x < middleOfSnap) {
        return lowerSnapTick;
      }

      if (x >= middleOfSnap) {
        return higherSnapTick;
      }
    }
  }
  return ticks.list.length;
}
