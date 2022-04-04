import { FC, useRef } from "react";
import { mdiDeleteOutline, mdiDrag } from "@mdi/js";
import merge from "classnames";

import { engine } from "../../../engine";

import { Icon } from "../../../ui/components/icon";
import { noop } from "../../../ui/utils/noop";
import { SortableItem } from "../../../ui/components/sortable-item";
import { Text } from "../../../components/text";

import "./styles.css";

interface Props {
  index: number;
  selected: boolean;
  instrumentKey: string;
  playerKey: string;

  onSelect: () => void;
}

export const InstrumentItem: FC<Props> = ({
  index,
  selected,
  instrumentKey,
  playerKey,
  onSelect,
}) => {
  const handle = useRef<HTMLDivElement>(null);
  const name = engine.get_instrument_name(playerKey, instrumentKey);

  return (
    <SortableItem
      handle={handle}
      index={index}
      className={merge("instrument-item", {
        "instrument-item--selected": selected,
      })}
    >
      <div ref={handle} onPointerDown={onSelect}>
        <Icon style={{ marginRight: 20 }} path={mdiDrag} size={24} />
      </div>
      <p className="instrument-item__name">
        <Text content={name} />
      </p>
      {selected && (
        <>
          <Icon
            aria-label="Remove Instrument"
            size={24}
            path={mdiDeleteOutline}
            onClick={noop}
          />
        </>
      )}
    </SortableItem>
  );
};
