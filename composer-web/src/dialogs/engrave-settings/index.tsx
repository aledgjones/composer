import { MenuItem } from "../../components/menu-item";
import { Dialog } from "../../ui/components/dialog";
import { Select } from "../../ui/components/select";
import { Option } from "../../ui/components/option";
import { Button } from "../../ui/components/button";
import { EngravePage } from "../../data/defs";
import { engine, ui } from "../../data";

import { Barlines } from "./barlines";
import { BracketsAndBraces } from "./brackets-and-braces";
import { NoteSpacing } from "./note-spacing";
import { Staves } from "./staves";

import "../generic-settings.css";

interface Props {
  onClose: () => void;
}

const getPage = (page: EngravePage, layoutKey: string) => {
  switch (page) {
    default:
    case EngravePage.Barlines:
      return <Barlines configKey={layoutKey} />;
    case EngravePage.BracketsAndBraces:
      return <BracketsAndBraces configKey={layoutKey} />;
    case EngravePage.NoteSpacing:
      return <NoteSpacing configKey={layoutKey} />;
    case EngravePage.Staves:
      return <Staves configKey={layoutKey} />;
  }
};

export const EngraveSettings = Dialog<Props>(({ onClose }) => {
  const configs: string[] = engine.engraves;
  const page = ui.useState((s) => s.setup.dialogs.engrave.page);
  const configKey =
    ui.useState((s) => s.setup.dialogs.engrave.config) || configs[0];

  return (
    <div className="generic-settings">
      <div className="generic-settings__content">
        <div className="generic-settings__left-panel">
          <MenuItem
            selected={page === EngravePage.Barlines}
            onClick={() => {
              ui.update((s) => {
                s.setup.dialogs.engrave.page = EngravePage.Barlines;
              });
            }}
          >
            Barlines
          </MenuItem>
          <MenuItem
            selected={page === EngravePage.BracketsAndBraces}
            onClick={() => {
              ui.update((s) => {
                s.setup.dialogs.engrave.page = EngravePage.BracketsAndBraces;
              });
            }}
          >
            Brackets &amp; Braces
          </MenuItem>
          <MenuItem
            selected={page === EngravePage.NoteSpacing}
            onClick={() => {
              ui.update((s) => {
                s.setup.dialogs.engrave.page = EngravePage.NoteSpacing;
              });
            }}
          >
            Note Spacing
          </MenuItem>
          <MenuItem
            selected={page === EngravePage.Staves}
            onClick={() => {
              ui.update((s) => {
                s.setup.dialogs.engrave.page = EngravePage.Staves;
              });
            }}
          >
            Staves
          </MenuItem>
        </div>

        <div className="generic-settings__right-panel">
          {getPage(page, configKey)}
        </div>
      </div>

      <div className="generic-settings__buttons">
        <Select
          direction="up"
          style={{ width: 300, marginRight: 8 }}
          value={configKey}
          onChange={(key: string) => {
            ui.update((s) => {
              s.setup.dialogs.engrave.config = key;
            });
          }}
        >
          {configs.map((config) => {
            return (
              <Option key={config} value={config} displayAs={config}>
                {config}
              </Option>
            );
          })}
        </Select>
        <Button disabled compact outline>
          Reset All
        </Button>
        <div className="generic-settings__spacer" />
        <Button compact onClick={onClose}>
          Close
        </Button>
      </div>
    </div>
  );
});
