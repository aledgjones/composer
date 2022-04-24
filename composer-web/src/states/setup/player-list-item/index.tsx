import { FC, useRef } from "react";
import {
  mdiAccount,
  mdiAccountGroup,
  mdiChevronDown,
  mdiChevronUp,
  mdiDeleteOutline,
  mdiPlus,
} from "@mdi/js";
import merge from "classnames";

import { SelectionType } from "../../../data/defs";
import { engine, store } from "../../../data";
import { PlayerType } from "composer-engine";
import { actions } from "../../../data/actions";

import { Icon } from "../../../ui/components/icon";
import { SortableContainer } from "../../../ui/components/sortable-container";
import { SortableItem } from "../../../ui/components/sortable-item";
import { Text } from "../../../components/text";

import { InstrumentItem } from "../instrument-item";

import "./styles.css";

const getIcon = (playerType: PlayerType) => {
  switch (playerType) {
    case PlayerType.Solo:
      return mdiAccount;
    default:
      return mdiAccountGroup;
  }
};

interface Props {
  index: number;
  playerKey: string;
  onAddInstrument: (playerKey: string) => void;
  onSelect: (key: string, type: SelectionType) => void;
  onClear: () => void;
}

export const PlayerItem: FC<Props> = ({
  index,
  playerKey,
  onAddInstrument,
  onSelect,
  onClear,
}) => {
  const handle = useRef<HTMLDivElement>(null);
  const expanded = store.useState(
    (s) => s.setup.expanded[playerKey],
    [playerKey]
  );
  const selected = store.useState(
    (s) => s.setup.selected?.key === playerKey,
    [playerKey]
  );

  const name: string = engine.get_player_name(playerKey);
  const type: PlayerType = engine.get_player_type(playerKey);
  const instruments: string[] = engine.get_player_instruments(playerKey);

  return (
    <SortableItem
      index={index}
      handle={handle}
      className={merge("player-item", {
        "player-item--selected": selected,
      })}
      onClick={() => onSelect(playerKey, SelectionType.Player)}
    >
      <div className="player-item__header">
        <div
          onPointerDown={() => onSelect(playerKey, SelectionType.Player)}
          ref={handle}
        >
          <Icon style={{ marginRight: 16 }} path={getIcon(type)} size={24} />
        </div>

        <p className="player-item__name">
          <Text content={name} />
        </p>

        {selected && (
          <>
            <Icon
              style={{ marginLeft: 12 }}
              size={24}
              path={mdiDeleteOutline}
              onClick={(e) => {
                e.stopPropagation();
                engine.remove_player(playerKey);
                onClear();
              }}
            />
            {(instruments.length === 0 || type === PlayerType.Solo) && (
              <Icon
                style={{ marginLeft: 12 }}
                path={mdiPlus}
                size={24}
                onClick={() => onAddInstrument(playerKey)}
              />
            )}
          </>
        )}
        <Icon
          style={{ marginLeft: 12 }}
          path={expanded ? mdiChevronUp : mdiChevronDown}
          size={24}
          onClick={(e) => {
            e.stopPropagation();
            actions.setup.expanded.toggle(playerKey);
          }}
        />
      </div>
      {expanded && (
        <SortableContainer
          direction="y"
          className="player-item__list"
          onEnd={(from, to) =>
            engine.reorder_player_instruments(playerKey, from, to)
          }
        >
          {instruments.map((instruemntKey, i) => {
            return (
              <InstrumentItem
                key={instruemntKey}
                index={i}
                playerKey={playerKey}
                instrumentKey={instruemntKey}
                selected={selected}
                onSelect={() => onSelect(playerKey, SelectionType.Player)}
              />
            );
          })}
        </SortableContainer>
      )}
    </SortableItem>
  );
};
