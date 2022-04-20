import { useState } from "react";
import { mdiAccount, mdiAccountGroup } from "@mdi/js";
import merge from "classnames";

import { PlayerType } from "composer-engine";

import { Button } from "../../ui/components/button";
import { Content } from "../../ui/components/content";
import { Dialog } from "../../ui/components/dialog";
import { Icon } from "../../ui/components/icon";
import { Label } from "../../ui/components/label";

import "../generic-settings.css";
import "./styles.css";

interface Props {
  onCancel: () => void;
  onSelect: (type: PlayerType) => void;
}

export const PlayerTypePicker = Dialog<Props>(({ onCancel, onSelect }) => {
  const [type, setType] = useState(PlayerType.Solo);

  return (
    <>
      <Content className="player-type-selector">
        <div
          className={merge("player-type-selector__box", {
            "player-type-selector__box--selected": type === PlayerType.Solo,
          })}
          onClick={() => setType(PlayerType.Solo)}
        >
          <Icon
            className="player-type-selector__icon"
            size={24}
            path={mdiAccount}
          />
          <Label>
            <p>Solo Player</p>
            <p>A single player who can hold mulpiple instruments</p>
          </Label>
        </div>
        <div
          className={merge("player-type-selector__box", {
            "player-type-selector__box--selected": type === PlayerType.Section,
          })}
          onClick={() => setType(PlayerType.Section)}
        >
          <Icon
            className="player-type-selector__icon"
            size={24}
            path={mdiAccountGroup}
          />
          <Label>
            <p>Section Player</p>
            <p>A group of players all with the same instrument</p>
          </Label>
        </div>
      </Content>
      <div className="generic-settings__buttons">
        <div className="generic-settings__spacer" />
        <Button
          compact
          style={{ marginRight: 8 }}
          outline
          onClick={() => onCancel()}
        >
          Cancel
        </Button>
        <Button compact onClick={() => onSelect(type)}>
          Next
        </Button>
      </div>
    </>
  );
});
