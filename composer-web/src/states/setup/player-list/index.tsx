import { FC, useState } from "react";
import { mdiCogOutline, mdiPlus } from "@mdi/js";

import { engine, store } from "../../../data";
import { SelectionType } from "../../../data/defs";
import { actions } from "../../../data/actions";

import { Icon } from "../../../ui/components/icon";
import { CollpaseDirection, Panel } from "../../../components/panel";
import { PanelHeader } from "../../../components/panel-header";
import { PlayerItem } from "../player-list-item";
import { SortableContainer } from "../../../ui/components/sortable-container";

import { PlayerSettings } from "../../../dialogs/player-settings";

import "./styles.css";

interface Props {
  onCreatePlayer: () => void;
  onAddInstrument: (playerKey: string) => void;
  onSelect: (key: string, type: SelectionType) => void;
  onClear: () => void;
}

export const PlayerList: FC<Props> = ({
  onCreatePlayer,
  onAddInstrument,
  onSelect,
  onClear,
}) => {
  const open = store.useState((s) => s.setup.panels.players);
  const [settings, setSettings] = useState(false);

  return (
    <>
      <Panel
        className="player-list"
        collapse={CollpaseDirection.Right}
        collapsed={!open}
        onToggle={actions.setup.panels.players.toggle}
      >
        <PanelHeader>
          <span className="player-list__label">Players</span>
          <Icon
            style={{ marginRight: 12 }}
            size={24}
            path={mdiCogOutline}
            onClick={() => setSettings(true)}
          />
          <Icon size={24} path={mdiPlus} onClick={onCreatePlayer} />
        </PanelHeader>
        <SortableContainer
          direction="y"
          className="player-list__content"
          onEnd={(from, to) => engine.reorder_players(from, to)}
        >
          {engine.players.map((playerKey: string, i: number) => {
            return (
              <PlayerItem
                index={i}
                key={playerKey}
                playerKey={playerKey}
                onAddInstrument={onAddInstrument}
                onSelect={onSelect}
                onClear={onClear}
              />
            );
          })}
        </SortableContainer>
      </Panel>
      <PlayerSettings
        width={900}
        open={settings}
        onClose={() => setSettings(false)}
      />
    </>
  );
};
