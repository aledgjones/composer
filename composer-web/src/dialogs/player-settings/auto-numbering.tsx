import { FC } from "react";

import { engine } from "../../data";
import { AutoCountStyle } from "composer-engine";

import { Option } from "../../ui/components/option";
import { Select } from "../../ui/components/select";
import { Subheader } from "../../ui/components/subheader";
import { Label } from "../../ui/components/label";

import "../generic-settings.css";

export const AutoNumbering: FC = () => {
  return (
    <>
      <div className="generic-settings__section">
        <Subheader>Numbering Style</Subheader>
        <Subheader subtle>Solo Player</Subheader>
        <Select
          margin
          value={engine.auto_count_style_solo}
          onChange={(value: AutoCountStyle) =>
            (engine.auto_count_style_solo = value)
          }
        >
          <Option value={AutoCountStyle.Arabic} displayAs="Arabic">
            <Label>
              <p>Arabic</p>
              <p>1, 2, 3...</p>
            </Label>
          </Option>
          <Option value={AutoCountStyle.Roman} displayAs="Roman">
            <Label>
              <p>Roman</p>
              <p>I, II, III...</p>
            </Label>
          </Option>
        </Select>
        <Subheader subtle>Section Player</Subheader>
        <Select
          value={engine.auto_count_style_section}
          onChange={(value: AutoCountStyle) =>
            (engine.auto_count_style_section = value)
          }
        >
          <Option value={AutoCountStyle.Arabic} displayAs="Arabic">
            <Label>
              <p>Arabic</p>
              <p>1, 2, 3...</p>
            </Label>
          </Option>
          <Option value={AutoCountStyle.Roman} displayAs="Roman">
            <Label>
              <p>Roman</p>
              <p>I, II, III...</p>
            </Label>
          </Option>
        </Select>
      </div>
    </>
  );
};
