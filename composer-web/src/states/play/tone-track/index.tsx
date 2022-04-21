import { FC, PointerEvent, useRef, useCallback } from "react";
import { ToneTrackEntry } from "../tone-track-entry";
import { SLOT_HEIGHT } from "../const";
import { getTickFromXPosition, getPitchFromYPosition, getStartOfTone, getDurationOfTone } from "../utils";
import { useStore } from "../../../store/use-store";
import { TickList } from "../../../store/score-flow/defs";
import { Tool } from "../../../store/ui/defs";
import { durationToTicks } from "../../../store/entries/time-signature/utils";
import { Tone } from "../../../store/entries/tone/defs";
import { EntryType, Articulation } from "../../../store/entries/defs";
import { actions } from "../../../store/actions";
import { pitchFromNumber } from "../../../store/entries/tone/utils";
import { dragHandler } from "../../../ui/utils/drag-handler";

import "./styles.css";

interface Props {
  flowKey: string;
  instrumentKey: string;
  color: string;
  ticks: TickList;
  base: number;
  tool: Tool;
  slots: number;
  zoom: number;
}

export const ToneTrack: FC<Props> = ({ flowKey, instrumentKey, color, base, tool, ticks, slots, zoom }) => {
  const track = useRef<HTMLDivElement>(null);

  const [audition, snap, tones, disabled, staveKey, trackKey] = useStore(
    (s) => {
      const flow = s.score.flows.byKey[flowKey];
      const instrument = s.score.instruments[instrumentKey];

      const staveKey = s.ui.play.stave[instrumentKey] || instrument.staves[0];
      const stave = flow.staves[staveKey];
      const trackKey = stave.tracks[0];

      const disabled = new Set();
      const tones = instrument.staves.reduce<Tone[]>((out, loopStaveKey) => {
        const isDisabled = s.ui.play.stave[instrumentKey] && loopStaveKey !== staveKey;
        flow.staves[loopStaveKey].tracks.forEach((trackKey) => {
          const track = s.score.tracks[trackKey];
          Object.values(track.entries.byKey).forEach((entry) => {
            if (entry.type === EntryType.Tone) {
              // only disable tones if there is a selected stave
              if (isDisabled) {
                disabled.add(entry.key);
              }
              out.push(entry as Tone);
            }
          });
        });
        return out;
      }, []);

      return [
        s.app.audition,
        durationToTicks(s.ui.snap, s.score.flows.byKey[flowKey].subdivisions),
        tones,
        disabled,
        staveKey,
        trackKey,
      ];
    },
    [flowKey, instrumentKey]
  );

  const onAudition = useCallback(
    (pitch: number) => {
      if (audition) {
        actions.playback.sampler.audition(instrumentKey, pitch);
      }
    },
    [instrumentKey, audition]
  );

  const onEdit = useCallback(
    (
      e: PointerEvent<HTMLElement>,
      toneKey: string,
      start: number,
      pitch: number,
      duration: number,
      articulation: Articulation,
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
            actions.score.entries.tone.pitch.set(trackKey, toneKey, pitchFromNumber(p));
          }

          if (!fixedStart) {
            const s = getStartOfTone(x, init.x, ticks, snap, zoom, start, duration, fixedStart, fixedDuration);
            actions.score.entries.tone.move(trackKey, toneKey, s);
          }

          if (!fixedDuration) {
            const d = getDurationOfTone(x, ticks, snap, zoom, start, duration, fixedStart, fixedDuration);
            actions.score.entries.tone.duration.set(trackKey, toneKey, d);
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

          if (!fixedStart) {
            const s = getStartOfTone(x, init.x, ticks, snap, zoom, start, duration, fixedStart, fixedDuration);
            actions.score.entries.tone.articulation.set(
              [{ key: toneKey, trackKey, tick: s, type: EntryType.Tone }],
              articulation
            );
          }

          if (!fixedDuration) {
            const d = getDurationOfTone(x, ticks, snap, zoom, start, duration, fixedStart, fixedDuration);
            if (d <= 0) {
              actions.score.entries.tone.remove(trackKey, toneKey);
            }
          }
        },
      });

      handler(e);
    },
    [flowKey, staveKey, instrumentKey, trackKey, track, ticks, base, slots, snap, audition, zoom, onAudition]
  );

  const onCreate = useCallback(
    (e: PointerEvent<HTMLDivElement>) => {
      if (track.current && tool === Tool.Draw) {
        const box = track.current.getBoundingClientRect();
        const x = e.clientX - box.left;
        const y = e.clientY - box.top;
        const start = getTickFromXPosition(x, ticks, snap, zoom, "down");
        const duration = getTickFromXPosition(x, ticks, snap, zoom) - start;
        const pitch = getPitchFromYPosition(y, base, slots);
        const tone = actions.score.entries.tone.create(
          trackKey,
          start,
          duration,
          pitchFromNumber(pitch),
          100,
          Articulation.None
        );

        actions.ui.selection.clear();
        actions.ui.selection.select(tone);

        onEdit(e, tone.key, start, duration, pitch, Articulation.None, true, false, true);
        onAudition(pitch);
      }

      if (tool === Tool.Select) {
        actions.ui.selection.clear();
      }
    },
    [flowKey, staveKey, instrumentKey, trackKey, track, ticks, base, slots, tool, snap, audition, zoom, onAudition]
  );

  const onSlice = useCallback(
    (e: PointerEvent<HTMLDivElement>, toneKey: string, start: number, duration: number) => {
      const box = track.current.getBoundingClientRect();
      const x = e.clientX - box.left;
      const slice = getTickFromXPosition(x, ticks, snap, zoom);

      if (slice > start && slice < start + duration) {
        actions.ui.selection.clear();
        actions.score.entries.tone.slice(trackKey, toneKey, slice);
      }
    },
    [trackKey, ticks, snap, zoom]
  );

  const onRemove = useCallback(
    (key: string) => {
      actions.ui.selection.clear();
      actions.score.entries.tone.remove(trackKey, key);
    },
    [trackKey]
  );

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
            color={color}
            base={base}
            slots={slots}
            tone={tone}
            ticks={ticks}
            tool={tool}
            zoom={zoom}
            onRemove={onRemove}
            onEdit={onEdit}
            onSlice={onSlice}
            onAudition={onAudition}
            disabled={disabled.has(tone.key)}
          />
        );
      })}
    </div>
  );
};
