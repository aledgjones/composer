import { FC, useState } from "react";
import { mdiCogOutline, mdiPlus } from "@mdi/js";

import { CollpaseDirection, Panel } from "../../../components/panel";
import { PanelHeader } from "../../../components/panel-header";
import { Icon } from "../../../ui/components/icon";
import { noop } from "../../../ui/utils/noop";
import { ui } from "../../../data";
import { EngraveSettings } from "../../../dialogs/engrave-settings";

import "./styles.css";

export const LayoutList: FC = () => {
  const [settings, setSettings] = useState(false);
  const open = ui.useState((s) => s.setup.panels.layouts);

  return (
    <>
      <Panel
        className="layout-list"
        collapse={CollpaseDirection.Left}
        collapsed={!open}
        onToggle={() => {
          ui.update((s) => {
            s.setup.panels.layouts = !s.setup.panels.layouts;
          });
        }}
      >
        <PanelHeader>
          <span className="layout-list__label">Layouts</span>
          <Icon
            style={{ marginRight: 12 }}
            size={24}
            path={mdiCogOutline}
            onClick={() => setSettings(true)}
          />
          <Icon disabled size={24} path={mdiPlus} onClick={noop} />
        </PanelHeader>
      </Panel>

      <EngraveSettings
        width={900}
        open={settings}
        onClose={() => setSettings(false)}
      />
    </>
  );
};
