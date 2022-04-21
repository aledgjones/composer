import { ui } from "../../data";
import { PlayerPage } from "../../data/defs";

import { MenuItem } from "../../components/menu-item";
import { Dialog } from "../../ui/components/dialog";
import { Button } from "../../ui/components/button";

import { AutoNumbering } from "./auto-numbering";

import "../generic-settings.css";

interface Props {
  onClose: () => void;
}

const getPage = (page: PlayerPage) => {
  switch (page) {
    default:
    case PlayerPage.AutoNumbering:
      return <AutoNumbering />;
  }
};

export const PlayerSettings = Dialog<Props>(({ onClose }) => {
  const page = ui.useState((s) => s.setup.dialogs.players.page);

  return (
    <div className="setup-settings generic-settings">
      <div className="generic-settings__content">
        <div className="generic-settings__left-panel">
          <MenuItem
            selected={page === PlayerPage.AutoNumbering}
            onClick={() =>
              ui.update((s) => {
                s.setup.dialogs.players.page = PlayerPage.AutoNumbering;
              })
            }
          >
            Auto Numbering
          </MenuItem>
        </div>

        <div className="generic-settings__right-panel">{getPage(page)}</div>
      </div>
      <div className="generic-settings__buttons">
        <div className="generic-settings__spacer" />
        <Button compact onClick={onClose}>
          Close
        </Button>
      </div>
    </div>
  );
});
