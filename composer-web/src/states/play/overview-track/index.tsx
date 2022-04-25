import { FC, useRef } from "react";
import Color from "color";
import { TickList } from "../ticks/defs";

import "./styles.css";
import { engine } from "../../../data";
import { Tone } from "../../../data/defs";

interface Props {
  flowKey: string;
  instrumentKey: string;
  color: string;
  ticks: TickList;
  zoom: number;
}

export const OverviewTrack: FC<Props> = ({
  flowKey,
  instrumentKey,
  color,
  ticks,
  zoom,
}) => {
  const track = useRef<HTMLDivElement>(null);
  const tones: Tone[] = engine.get_all_tones(flowKey, instrumentKey);

  const blocks = tones.reduce<[number, number][]>((out, tone) => {
    const prev = out[out.length - 1];
    const start = tone.tick;
    const stop = tone.tick + tone.duration;
    if (prev) {
      const [, prevStop] = prev;
      if (start < prevStop && stop < prevStop) {
        // the tone is within the previous range so ignore
      } else if (start > prevStop) {
        // the tone lays outside the previous range so start a new range
        out.push([start, stop]);
      } else if (stop > prevStop) {
        // the tone starts in the previous range but carries on longer so extend the range
        prev[1] = stop;
      }
    } else {
      out.push([start, stop]);
    }

    return out;
  }, []);

  return (
    <div
      ref={track}
      className="overview-track"
      style={{ width: ticks.width * zoom }}
    >
      {blocks.map(([start, stop], i) => {
        return (
          <div
            key={i}
            className="overview-track__block"
            style={{
              backgroundColor: Color(color).alpha(0.1).toString(),
              left: ticks.list[start].x * zoom,
              width: ticks.list[stop]
                ? (ticks.list[stop].x - ticks.list[start].x) * zoom
                : (ticks.width - ticks.list[start].x) * zoom,
            }}
          />
        );
      })}
      {tones.map((tone) => {
        const start = tone.tick;
        const stop = tone.tick + tone.duration;
        return (
          <div
            key={tone.key}
            className="overview-track__tone"
            style={{
              position: "absolute",
              backgroundColor: color,
              left: ticks.list[start].x * zoom,
              width: ticks.list[stop]
                ? (ticks.list[stop].x - ticks.list[start].x) * zoom
                : (ticks.width - ticks.list[start].x) * zoom,
              height: `calc(100% / 100)`,
              bottom: `calc(1% * ${tone.pitch.int - 12})`, // C0 -> E8 (12 -> 112)
            }}
          />
        );
      })}
    </div>
  );
};
