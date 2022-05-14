import { FC } from "react";
import { SLOT_COUNT, SLOT_HEIGHT } from "../const";
import { Ticks } from "../ticks";
import { Slots } from "../keyboard/slots";
import { ToneTrack } from "../tone-track";
import merge from "classnames";
import { engine, store } from "../../../data";
import { TickList } from "../ticks/defs";
import { Tool } from "../../../data/defs";
import { OverviewTrack } from "../overview-track";

import "./styles.css";

interface Props {
  flowKey: string;
  instrumentKey: string;
  color: string;
  ticks: TickList;
  tool: Tool;
  zoom: number;
}

export const Track: FC<Props> = ({
  flowKey,
  instrumentKey,
  color,
  ticks,
  tool,
  zoom,
}) => {
  const expanded = store.useState(
    (s) => s.play.expanded[instrumentKey],
    [instrumentKey]
  );
  const tracks: [string, string][] = engine.get_instrument_tracks(
    flowKey,
    instrumentKey
  );
  const trackKey = store.useState(
    (s) => s.play.track[instrumentKey] || tracks[0]?.[0],
    [instrumentKey, tracks]
  );
  const base = store.useState(
    (s) => s.play.keyboard[trackKey] || 76,
    [trackKey]
  );

  return (
    <div className="track">
      <Ticks
        isTrack={true}
        ticks={ticks}
        height={48}
        className="track__header"
        zoom={zoom}
      />
      {!expanded && (
        <OverviewTrack
          color={color}
          flowKey={flowKey}
          instrumentKey={instrumentKey}
          ticks={ticks}
          zoom={zoom}
        />
      )}
      {expanded && (
        <>
          <div
            className={merge("track__tone-channel", {
              "no-scroll": tool !== Tool.Select,
            })}
            style={{ height: SLOT_HEIGHT * SLOT_COUNT }}
          >
            <Slots
              style={{ width: ticks.width * zoom }}
              className="track__tone-channel-slots"
              base={base}
              count={SLOT_COUNT}
              isKeyboard={false}
            />
            <Ticks
              isTrack={true}
              className="track__tone-channel-ticks"
              ticks={ticks}
              height={SLOT_HEIGHT * SLOT_COUNT}
              zoom={zoom}
            />
            <ToneTrack
              color={color}
              trackKey={trackKey}
              ticks={ticks}
              base={base}
              tool={tool}
              slots={SLOT_COUNT}
              zoom={zoom}
            />
          </div>
        </>
      )}
    </div>
  );
};
