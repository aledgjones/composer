import { FC, CSSProperties } from "react";
import merge from "classnames";
import { modulo } from "../../../ui/utils/modulo";
import { SLOT_HEIGHT } from "../const";

interface Props {
  className?: string;
  style?: CSSProperties;
  base: number; // MIDIPitch
  count: number; // number of slots
  isKeyboard: boolean;
}

export const Slots: FC<Props> = ({ className, style, base, count, isKeyboard }) => {
  const basePattern = [0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0];
  const basePatternOffset = 76; // E5
  const offset = basePatternOffset - base;

  const whiteKeySpace = (SLOT_HEIGHT * 12) / 7;
  const blackKeySpace = SLOT_HEIGHT;

  const pattern = [];
  for (let i = 0; i < count; i++) {
    const index = modulo(offset + i, basePattern.length);
    pattern.push(basePattern[index]);
  }

  return (
    <svg
      className={merge("slots", className)}
      style={style}
      width="64"
      height={SLOT_HEIGHT * count}
      xmlns="https://www.w3.org/2000/svg"
      viewBox={`0 0 64 ${SLOT_HEIGHT * count}`}
      preserveAspectRatio={isKeyboard ? undefined : "none"}
    >
      {/* White Keys */}
      {Array(pattern.reduce((out, val) => out - val, count + 2))
        .fill(null)
        .map((val, i) => {
          return (
            <rect
              key={i}
              fill={isKeyboard ? "#ffffff" : "var(--background-800)"}
              x={isKeyboard ? -3 : 0}
              y={i * whiteKeySpace - modulo(offset * SLOT_HEIGHT, whiteKeySpace)}
              width={isKeyboard ? 67 : 64}
              height={whiteKeySpace - 1}
              rx={isKeyboard ? 3 : 0}
            />
          );
        })}

      {/* Black Keys */}
      {pattern.map((val, i) => {
        if (val === 1) {
          return (
            <rect
              key={i}
              fill={isKeyboard ? "var(--black)" : "var(--background-700)"}
              x={isKeyboard ? -3 : 0}
              y={blackKeySpace * i}
              width={isKeyboard ? 47 : 64}
              height={blackKeySpace}
              rx={isKeyboard ? 2.5 : 0}
            />
          );
        } else {
          return null;
        }
      })}

      {/* Labels */}
      {isKeyboard &&
        Array(9)
          .fill(null)
          .map((val, i) => {
            return (
              <text
                key={i}
                fontSize="10"
                fill="var(--black)"
                alignmentBaseline="central"
                x="48"
                y={
                  i * (12 * SLOT_HEIGHT) + // space an octave apart
                  2.5 * whiteKeySpace - // shift to the C
                  3 * (SLOT_HEIGHT * 12) - // offset all so C5 is in the correct octave
                  offset * SLOT_HEIGHT // make it move
                }
              >
                C{8 - i}
              </text>
            );
          })}
    </svg>
  );
};
