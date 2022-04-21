import { FC } from "react";
import { SLOT_COUNT, SLOT_HEIGHT } from "../const";
import { Ticks } from "../ticks";
import { Slots } from "../keyboard/slots";
import { ToneTrack } from "../tone-track";
import { OverviewTrack } from "../overview-track";
import { useStore } from "../../../store/use-store";
import { TickList } from "../../../store/score-flow/defs";
import { Tool } from "../../../store/ui/defs";
import merge from "classnames";

import "./styles.css";

interface Props {
  flowKey: string;
  instrumentKey: string;
  color: string;
  ticks: TickList;
  zoom: number;
}

export const Track: FC<Props> = ({ flowKey, instrumentKey, color, ticks, zoom }) => {
  const [expanded, tool, base] = useStore(
    (s) => [s.ui.play.expanded[instrumentKey], s.ui.play.tool, s.ui.play.keyboard[instrumentKey] || 76],
    [instrumentKey]
  );

  return (
    <div className="track">
      <Ticks isTrack={true} ticks={ticks} height={48} className="track__header" zoom={zoom} />
      {!expanded && (
        <OverviewTrack color={color} flowKey={flowKey} instrumentKey={instrumentKey} ticks={ticks} zoom={zoom} />
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
              flowKey={flowKey}
              instrumentKey={instrumentKey}
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
