import { FC, useMemo, PointerEvent, useCallback } from "react";
import { SLOT_HEIGHT } from "../const";
import { useStore } from "../../../store/use-store";
import { Tone } from "../../../store/entries/tone/defs";
import { TickList } from "../../../store/score-flow/defs";
import { Tool } from "../../../store/ui/defs";
import { actions } from "../../../store/actions";
import merge from "classnames";
import { Articulation } from "../../../store/entries/defs";

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
  color: string;
  base: number;
  slots: number;
  tone: Tone;
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
    articulation: Articulation,
    fixedStart: boolean,
    fixedDuration: boolean,
    fixedPitch: boolean
  ) => void;
  onSlice: (e: PointerEvent<HTMLElement>, toneKey: string, start: number, duration: number) => void;
  onAudition: (pitch: number) => void;
  disabled: boolean;
}

export const ToneTrackEntry: FC<Props> = ({
  color,
  base,
  slots,
  tone,
  ticks,
  tool,
  zoom,
  onRemove,
  onEdit,
  onSlice,
  onAudition,
  disabled,
}) => {
  const selected = useStore((s) => Boolean(s.ui.selection.find((entry) => entry.key === tone.key)), [tone.key]);

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

  const actionMain = useCallback(
    (e: PointerEvent<HTMLDivElement>) => {
      // stop deselection on track
      e.stopPropagation();

      if (tool === Tool.Select && !selected) {
        actions.ui.selection.clear();
        actions.ui.selection.select(tone);
        onAudition(tone.pitch.int);
      }
      if (tool === Tool.Erase) {
        onRemove(tone.key);
      }
      if (tool === Tool.Slice) {
        onSlice(e, tone.key, tone.tick, tone.duration);
      }
    },
    [tone, onRemove, onSlice, tool, selected, onAudition]
  );

  const actionWest = useCallback(
    (e: PointerEvent<HTMLElement>) =>
      onEdit(e, tone.key, tone.tick, tone.pitch.int, tone.duration, tone.articulation, false, false, true),
    [tone, onEdit]
  );

  const action = useCallback(
    (e: PointerEvent<HTMLElement>) =>
      onEdit(e, tone.key, tone.tick, tone.pitch.int, tone.duration, tone.articulation, false, true, false),
    [tone, onEdit]
  );

  const actionEast = useCallback(
    (e: PointerEvent<HTMLElement>) =>
      onEdit(e, tone.key, tone.tick, tone.pitch.int, tone.duration, tone.articulation, true, false, true),
    [tone, onEdit]
  );

  if (shouldDraw(tone.pitch.int, base, slots)) {
    return (
      <div
        className={merge("tone-track-entry", "no-scroll", {
          "tone-track-entry--selected": !!selected,
          "tone-track-entry--disabled": disabled,
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
            <div className="tone-track-entry__handle tone-track-entry__handle--w" onPointerDown={actionWest} />
            <div className="tone-track-entry__handle tone-track-entry__handle--move" onPointerDown={action} />
            <div className="tone-track-entry__handle tone-track-entry__handle--e" onPointerDown={actionEast} />
          </>
        )}
      </div>
    );
  } else {
    return null;
  }
};
