import { FC, useMemo, PointerEvent } from "react";
import { SLOT_HEIGHT } from "../const";
import merge from "classnames";
import { TickList } from "../ticks/defs";
import { Tone, Tool } from "../../../data/defs";
import { actions } from "../../../data/actions";
import { store } from "../../../data";

import "./styles.css";

// TODO: write a should render function: is ticks in view?
function shouldDraw(pitch: number, base: number, slots: number) {
  if (pitch > base) {
    return false;
  }

  if (pitch <= base - slots) {
    return false;
  }

  return true;
}

interface Props {
  trackKey: string;
  tone: Tone;
  color: string;
  base: number;
  slots: number;
  ticks: TickList;
  tool: Tool;
  zoom: number;
  onRemove: (key: string) => void;
  onEdit: (
    e: PointerEvent<HTMLElement>,
    toneKey: string,
    start: number,
    pitch: number,
    duration: number,
    fixedStart: boolean,
    fixedDuration: boolean,
    fixedPitch: boolean
  ) => void;
  onSlice: (
    e: PointerEvent<HTMLDivElement>,
    toneKey: string,
    start: number,
    duration: number
  ) => void;
  onAudition: (pitch: number) => void;
}

export const ToneTrackEntry: FC<Props> = ({
  trackKey,
  tone,
  color,
  base,
  slots,
  ticks,
  tool,
  zoom,
  onRemove,
  onEdit,
  onSlice,
  onAudition,
}) => {
  const selected = store.useState(
    (s) => Boolean(s.selection.find((entry) => entry.key === tone.key)),
    [tone.key]
  );

  const left = useMemo(() => {
    if (tone.tick >= ticks.list.length) {
      return ticks.width * zoom;
    } else {
      return ticks.list[tone.tick].x * zoom;
    }
  }, [tone, ticks, zoom]);

  const width = useMemo(() => {
    if (tone.tick + tone.duration >= ticks.list.length) {
      return ticks.width * zoom - left;
    } else {
      return ticks.list[tone.tick + tone.duration].x * zoom - left;
    }
  }, [tone, ticks, left, zoom]);

  const actionMain = (e: PointerEvent<HTMLDivElement>) => {
    // stop deselection on track
    e.stopPropagation();

    if (tool === Tool.Select && !selected) {
      actions.ui.selection.clear();
      actions.ui.selection.select({ trackKey, key: tone.key });
      onAudition(tone.pitch.int);
    }
    if (tool === Tool.Erase) {
      onRemove(tone.key);
    }
    if (tool === Tool.Slice) {
      onSlice(e, tone.key, tone.tick, tone.duration);
    }
  };

  const actionWest = (e: PointerEvent<HTMLElement>) => {
    onEdit(
      e,
      tone.key,
      tone.tick,
      tone.pitch.int,
      tone.duration,
      false,
      false,
      true
    );
  };

  const action = (e: PointerEvent<HTMLElement>) => {
    onEdit(
      e,
      tone.key,
      tone.tick,
      tone.pitch.int,
      tone.duration,
      false,
      true,
      false
    );
  };

  const actionEast = (e: PointerEvent<HTMLElement>) => {
    onEdit(
      e,
      tone.key,
      tone.tick,
      tone.pitch.int,
      tone.duration,
      true,
      false,
      true
    );
  };

  if (shouldDraw(tone.pitch.int, base, slots)) {
    return (
      <div
        className={merge("tone-track-entry", "no-scroll", {
          "tone-track-entry--selected": !!selected,
        })}
        style={{
          position: "absolute",
          top: (base - tone.pitch.int) * SLOT_HEIGHT,
          left,
          width,
          height: SLOT_HEIGHT,
          backgroundColor: color,
        }}
        onPointerDown={actionMain}
      >
        {tool === Tool.Select && (
          <>
            <div
              className="tone-track-entry__handle tone-track-entry__handle--w"
              onPointerDown={actionWest}
            />
            <div
              className="tone-track-entry__handle tone-track-entry__handle--move"
              onPointerDown={action}
            />
            <div
              className="tone-track-entry__handle tone-track-entry__handle--e"
              onPointerDown={actionEast}
            />
          </>
        )}
      </div>
    );
  } else {
    return null;
  }
};
