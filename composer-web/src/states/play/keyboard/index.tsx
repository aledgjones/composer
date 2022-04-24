import { FC } from "react";
import { Slots } from "./slots";
import { SLOT_HEIGHT } from "../const";
import { useDragHandler } from "../../../ui/hooks/use-drag-handler";
import { store } from "../../../data";
import { noop } from "../../../ui/utils/noop";
import { actions } from "../../../data/actions";

import "./styles.css";

interface Props {
  trackKey: string;
  height: number;
}

export const Keyboard: FC<Props> = ({ trackKey, height }) => {
  const base = store.useState(
    (s) => s.play.keyboard[trackKey] || 76,
    [trackKey]
  );

  const onDrag = useDragHandler<{ y: number; base: number }>(
    {
      onDown: (e) => {
        return {
          y: e.screenY,
          base,
        };
      },
      onMove: (e, init) => {
        const change = Math.round((init.y - e.screenY) / SLOT_HEIGHT);
        const next = init.base - change;
        // E8 <= next >= E1
        if (next <= 112 && next >= 28) {
          actions.play.keyboard.set(trackKey, init.base - change);
        }
      },
      onEnd: noop,
    },
    [base, trackKey]
  );

  return (
    <div
      className="keyboard"
      onPointerDown={onDrag}
      style={{ height: height * SLOT_HEIGHT }}
    >
      <Slots base={base} count={height} isKeyboard={true} />
    </div>
  );
};
