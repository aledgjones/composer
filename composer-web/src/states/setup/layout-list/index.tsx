import { FC } from "react";
import { mdiCogOutline, mdiPlus } from "@mdi/js";

import { CollpaseDirection, Panel } from "../../../components/panel";
import { PanelHeader } from "../../../components/panel-header";
import { Icon } from "../../../ui/components/icon";

import "./styles.css";

import { noop } from "../../../ui/utils/noop";
import { ui } from "../../../data";

export const LayoutList: FC = () => {
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
            disabled
            style={{ marginRight: 12 }}
            size={24}
            path={mdiCogOutline}
            onClick={noop}
          />
          <Icon disabled size={24} path={mdiPlus} onClick={noop} />
        </PanelHeader>
      </Panel>
    </>
  );
};
