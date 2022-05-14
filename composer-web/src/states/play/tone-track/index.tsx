import { FC, PointerEvent, useRef } from "react";
import { SLOT_HEIGHT } from "../const";
import {
  getTickFromXPosition,
  getPitchFromYPosition,
  getStartOfTone,
  getDurationOfTone,
} from "../utils";
import { dragHandler } from "../../../ui/utils/drag-handler";
import { TickList } from "../ticks/defs";
import { Tone, Tool } from "../../../data/defs";
import { engine, store } from "../../../data";
import { actions } from "../../../data/actions";
import { Articulation } from "composer-engine";
import { ToneTrackEntry } from "../tone-track-entry";

import "./styles.css";

interface Props {
  trackKey: string;
  color: string;
  ticks: TickList;
  base: number;
  tool: Tool;
  slots: number;
  zoom: number;
}

export const ToneTrack: FC<Props> = ({
  trackKey,
  color,
  base,
  tool,
  ticks,
  slots,
  zoom,
}) => {
  const track = useRef<HTMLDivElement>(null);
  const snap = store.useState((s) => s.snap);
  const audition = store.useState((s) => s.audition);
  const tones: Tone[] = engine.get_tones(trackKey);

  const onAudition = (pitch: number) => {
    if (audition) {
      // TODO: playback
      // actions.playback.sampler.audition(instrumentKey, pitch);
    }
  };

  const onEdit = (
    e: PointerEvent<HTMLElement>,
    toneKey: string,
    start: number,
    pitch: number,
    duration: number,
    fixedStart: boolean,
    fixedDuration: boolean,
    fixedPitch: boolean
  ) => {
    const handler = dragHandler<{ box: DOMRect; x: number }>({
      onDown: (ev) => {
        if (track.current) {
          const box = track.current.getBoundingClientRect();
          return {
            box,
            x: ev.clientX - box.left,
          };
        } else {
          return false;
        }
      },
      onMove: (ev, init) => {
        const x = ev.clientX - init.box.left;

        if (!fixedPitch) {
          const y = ev.clientY - init.box.top;
          const p = getPitchFromYPosition(y, base, slots);
          engine.set_tone_pitch(trackKey, toneKey, p);
        }

        if (!fixedStart) {
          const s = getStartOfTone(
            x,
            init.x,
            ticks,
            snap,
            zoom,
            start,
            duration,
            fixedStart,
            fixedDuration
          );
          engine.shift_tone(trackKey, toneKey, s);
        }

        if (!fixedDuration) {
          const d = getDurationOfTone(
            x,
            ticks,
            snap,
            zoom,
            start,
            duration,
            fixedStart,
            fixedDuration
          );
          engine.set_tone_duration(trackKey, toneKey, d);
        }
      },
      onEnd: (ev, init) => {
        const x = ev.clientX - init.box.left;

        if (!fixedPitch) {
          const y = ev.clientY - init.box.top;
          const p = getPitchFromYPosition(y, base, slots);

          if (p !== pitch) {
            onAudition(p);
          }
        }

        if (!fixedDuration) {
          const d = getDurationOfTone(
            x,
            ticks,
            snap,
            zoom,
            start,
            duration,
            fixedStart,
            fixedDuration
          );
          if (d <= 0) {
            engine.remove_tone(trackKey, toneKey);
          }
        }
      },
    });

    handler(e);
  };

  const onCreate = (e: PointerEvent<HTMLDivElement>) => {
    if (track.current && tool === Tool.Draw) {
      const box = track.current.getBoundingClientRect();
      const x = e.clientX - box.left;
      const y = e.clientY - box.top;
      const start = getTickFromXPosition(x, ticks, snap, zoom, "down");
      const duration = getTickFromXPosition(x, ticks, snap, zoom) - start;
      const pitch = getPitchFromYPosition(y, base, slots);
      const toneKey = engine.create_tone(
        trackKey,
        start,
        duration,
        pitch,
        undefined,
        100,
        Articulation.None
      );

      actions.ui.selection.clear();
      actions.ui.selection.select({ key: toneKey, trackKey });

      onEdit(e, toneKey, start, duration, pitch, true, false, true);
      onAudition(pitch);
    }

    if (tool === Tool.Select) {
      actions.ui.selection.clear();
    }
  };

  const onSlice = (
    e: PointerEvent<HTMLDivElement>,
    toneKey: string,
    start: number,
    duration: number
  ) => {
    if (track.current) {
      const box = track.current.getBoundingClientRect();
      const x = e.clientX - box.left;
      const sliceAt = getTickFromXPosition(x, ticks, snap, zoom);

      if (sliceAt > start && sliceAt < start + duration) {
        actions.ui.selection.clear();
        engine.slice_tone(trackKey, toneKey, sliceAt);
      }
    }
  };

  const onRemove = (key: string) => {
    actions.ui.selection.clear();
    engine.remove_tone(trackKey, key);
  };

  return (
    <div
      ref={track}
      onPointerDown={onCreate}
      className="tone-track"
      style={{ width: ticks.width * zoom, height: SLOT_HEIGHT * slots }}
    >
      {tones.map((tone) => {
        return (
          <ToneTrackEntry
            key={tone.key}
            trackKey={trackKey}
            tone={tone}
            color={color}
            base={base}
            slots={slots}
            ticks={ticks}
            tool={tool}
            zoom={zoom}
            onRemove={onRemove}
            onEdit={onEdit}
            onSlice={onSlice}
            onAudition={onAudition}
          />
        );
      })}
    </div>
  );
};
