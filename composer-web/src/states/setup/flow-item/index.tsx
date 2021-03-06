import { MouseEvent, FC, useRef, CSSProperties, useState } from "react";
import {
  mdiDeleteOutline,
  mdiFileDocumentOutline,
  mdiPencilOutline,
} from "@mdi/js";
import merge from "classnames";

import { engine, store } from "../../../data";
import { SelectionType } from "../../../data/defs";

import { SortableItem } from "../../../ui/components/sortable-item";
import { Icon } from "../../../ui/components/icon";
import { Checkbox } from "../../../ui/components/checkbox";

import "./styles.css";

interface Props {
  index: number;
  flowKey: string;
  style: CSSProperties;
  onSelect: (key: string, type: SelectionType) => void;
  onClear: () => void;
}

export const FlowItem: FC<Props> = ({
  index,
  flowKey,
  style,
  onSelect,
  onClear,
}) => {
  const handle = useRef<HTMLDivElement>(null);
  const input = useRef<HTMLInputElement>(null);

  const title = engine.get_flow_title(flowKey);

  const [savedValue, setSavedValue] = useState(title);
  const [editing, setEditing] = useState(false);
  const selection = store.useState((s) => s.setup.selected);
  const selected = selection?.key === flowKey;
  const active = selection
    ? engine.flow_contains_player(flowKey, selection.key)
    : false;

  const onCheckboxChange = (value: boolean) => {
    if (selection) {
      const playerKey = selection.key;
      if (value) {
        engine.assign_player_to_flow(flowKey, playerKey);
      } else {
        engine.unassign_player_from_flow(flowKey, playerKey);
      }
    }
  };

  const onRemove = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    engine.remove_flow(flowKey);
    onClear();
  };

  const onEdit = () => {
    if (input.current) {
      setSavedValue(title);
      input.current.focus();
    }
    setEditing(true);
  };

  const onKeyDown = (e: any) => {
    switch (e.key) {
      case "Enter":
        // confirm
        if (input.current) {
          input.current.blur();
        }
        break;
      case "Escape":
        // revert
        engine.rename_flow(flowKey, savedValue);
        input.current?.blur();
        break;
      default:
        break;
    }
  };

  return (
    <SortableItem
      index={index}
      handle={handle}
      className={merge("flow-item", {
        "flow-item--editing": editing,
        "flow-item--selected": selected,
        "flow-item--active": active,
      })}
      style={style}
      onClick={() => onSelect(flowKey, SelectionType.Flow)}
    >
      <div className="flow-item__header">
        <div
          onPointerDown={() => onSelect(flowKey, SelectionType.Flow)}
          ref={handle}
        >
          <Icon
            style={{ marginRight: 12 }}
            path={mdiFileDocumentOutline}
            size={24}
          />
        </div>

        <input
          ref={input}
          style={{ fontStyle: !editing && !title ? "italic" : "normal" }}
          onBlur={() => setEditing(false)}
          readOnly={!editing}
          className="flow-item__name"
          tabIndex={editing ? 0 : -1}
          value={editing ? title : title || "Untitled Flow"}
          onKeyDown={onKeyDown}
          onInput={(e: any) => engine.rename_flow(flowKey, e.target.value)}
        />

        {selected && (
          <>
            <Icon
              style={{ marginLeft: 12 }}
              size={24}
              path={mdiPencilOutline}
              onClick={onEdit}
            />
            <Icon
              style={{ marginLeft: 12 }}
              size={24}
              path={mdiDeleteOutline}
              onClick={onRemove}
            />
          </>
        )}

        {selection && selection.type !== SelectionType.Flow && (
          <div onClick={(e) => e.stopPropagation()}>
            <Checkbox value={active} onChange={onCheckboxChange} />
          </div>
        )}
      </div>
    </SortableItem>
  );
};
