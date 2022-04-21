import { FC, useCallback, KeyboardEvent, useRef } from "react";
// import { useWaveform } from "../../../store/playback/utils";
import { useDragHandler } from "../../../ui/hooks/use-drag-handler";
import merge from "classnames";
import { noop } from "../../../ui/utils/noop";

import "./styles.css";

interface Props {
  instrumentKey: string;
  volume: number;
  color: string;
  onChange: (value: number) => void;
}

export const Fader: FC<Props> = ({
  instrumentKey,
  volume,
  color,
  onChange,
}) => {
  const ref = useRef<HTMLDivElement>(null);
  // const amplitude = useWaveform(instrumentKey);

  const keyPress = useCallback(
    (e: KeyboardEvent<HTMLButtonElement>) => {
      if (e.key === "ArrowLeft") {
        e.preventDefault();
        volume - 1 < 0 ? onChange(0) : onChange(volume - 1);
      }
      if (e.key === "ArrowRight") {
        e.preventDefault();
        volume + 1 > 100 ? onChange(100) : onChange(volume + 1);
      }
    },
    [volume, onChange]
  );

  const pointerDown = useDragHandler<{ width: number; left: number }>(
    {
      onDown: () => {
        if (!ref.current) {
          return false;
        }

        const box = ref.current.getBoundingClientRect();
        return { width: box.width, left: box.left };
      },
      onMove: (e, data) => {
        const offset = e.clientX - data.left;
        if (offset < 0) {
          onChange(0);
        } else if (offset > data.width) {
          onChange(100);
        } else {
          const next = (offset / data.width) * 100;
          onChange(next);
        }
      },
      onEnd: noop,
    },
    [ref]
  );

  return (
    <div className="fader">
      <div className="fader__track" ref={ref}>
        <button
          className="fader__handle"
          onPointerDown={pointerDown}
          onKeyDown={keyPress}
          style={{ left: `${volume}%`, backgroundColor: color }}
        />
        <div
          className="fader__meter"
          style={{
            backgroundColor: color,
            // width: `${amplitude * 100}%`,
          }}
        />
      </div>
      {Array(11)
        .fill(null)
        .map((val, i) => {
          return (
            <div
              key={i}
              className={merge("fader__scale", {
                "fader__scale--tall": i % 5 === 0,
              })}
              style={{ left: `${i * 10}%` }}
            />
          );
        })}
    </div>
  );
};
