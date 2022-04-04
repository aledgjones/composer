import { FC } from "react";
import { mdiCogOutline, mdiPlus } from "@mdi/js";

import { engine } from "../../../engine";
import { ui } from "../../../store";

import { Icon } from "../../../ui/components/icon";
import { CollpaseDirection, Panel } from "../../../components/panel";
import { PanelHeader } from "../../../components/panel-header";
import { PlayerItem } from "../player-list-item";
import { SortableContainer } from "../../../ui/components/sortable-container";

import "./styles.css";

interface Props {
  onCreatePlayer: () => void;
  onAddInstrument: (playerKey: string) => void;
}

export const PlayerList: FC<Props> = ({ onCreatePlayer, onAddInstrument }) => {
  const open = ui.useState((s) => s.setup.panels.players);

  return (
    <>
      <Panel
        className="player-list"
        collapse={CollpaseDirection.Right}
        collapsed={!open}
        onToggle={() => {
          ui.update((s) => {
            s.setup.panels.players = !s.setup.panels.players;
          });
        }}
      >
        <PanelHeader>
          <span className="player-list__label">Players</span>
          <Icon
            style={{ marginRight: 12 }}
            size={24}
            path={mdiCogOutline}
            // onClick={() => setSettings(true)}
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
              />
            );
          })}
        </SortableContainer>
      </Panel>
    </>
  );
};
