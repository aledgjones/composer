import { FC } from "react";
import { mdiPlus } from "@mdi/js";

import { engine } from "../../../engine";
import { SelectionType } from "../../../store/defs";
import { ui } from "../../../store";

import { Panel } from "../../../components/panel";
import { PanelHeader } from "../../../components/panel-header";
import { Icon } from "../../../ui/components/icon";
import { SortableContainer } from "../../../ui/components/sortable-container";

import { FlowItem } from "../flow-item";

import "./styles.css";

export const FlowList: FC = () => {
  const width = `calc(${100 / engine.flows.length}% - 8px)`;

  const onSelect = (key: string) => {
    ui.update((s) => {
      s.setup.selected = { key, type: SelectionType.Flow };
    });
  };

  return (
    <Panel className="flow-list">
      <PanelHeader>
        <span className="flow-list__label">Flows</span>
        <Icon
          size={24}
          path={mdiPlus}
          onClick={() => {
            const key = engine.create_flow();
            onSelect(key);
          }}
        />
      </PanelHeader>
      <div className="flow-list__wrapper">
        <SortableContainer
          direction="x"
          className="flow-list__content"
          onEnd={(from: number, to: number) => {
            console.log(from, to);
            engine.reorder_flow(from, to);
          }}
        >
          {engine.flows.map((flowKey: string, i: number) => (
            <FlowItem
              index={i}
              key={flowKey}
              flowKey={flowKey}
              style={{ width }}
            />
          ))}
        </SortableContainer>
      </div>
    </Panel>
  );
};
