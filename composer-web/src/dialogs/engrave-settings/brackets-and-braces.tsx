import { BracketingApproach, BracketStyle } from "composer-engine";
import { FC } from "react";
import { engine } from "../../data";
import { Label } from "../../ui/components/label";
import { ListItem } from "../../ui/components/list-item";
import { Option } from "../../ui/components/option";
import { Select } from "../../ui/components/select";
import { Subheader } from "../../ui/components/subheader";
import { Switch } from "../../ui/components/switch";

interface Props {
  configKey: string;
}

export const BracketsAndBraces: FC<Props> = ({ configKey }) => {
  const bracketingApproach = engine.get_bracketing_approach(configKey);
  const bracketSingleStaves = engine.get_bracket_single_staves(configKey);
  const subBracket = engine.get_sub_bracket(configKey);
  const bracketStyle = engine.get_bracket_style(configKey);

  console.log(bracketingApproach);

  return (
    <>
      <div className="generic-settings__section" style={{ paddingBottom: 20 }}>
        <Subheader>Approach</Subheader>
        <Select
          className="ui-select--margin"
          value={bracketingApproach}
          onChange={(value: BracketingApproach) => {
            engine.set_bracketing_approach(configKey, value);
          }}
        >
          <Option value={BracketingApproach.None} displayAs="None">
            None
          </Option>
          <Option value={BracketingApproach.Orchestral} displayAs="Orchestral">
            Orchestral
          </Option>
          <Option
            value={BracketingApproach.SmallEnsemble}
            displayAs="Small ensemble"
          >
            Small ensemble
          </Option>
        </Select>
      </div>
      <ListItem
        disabled={bracketingApproach === BracketingApproach.None}
        onClick={() => {
          engine.set_bracket_single_staves(configKey, !bracketSingleStaves);
        }}
      >
        <Label>
          <p>Bracket single instruments.</p>
          <p>
            Use a bracket for isolated instruments of a particular instrument
            family.
          </p>
        </Label>
        <Switch value={bracketSingleStaves} />
      </ListItem>
      <ListItem
        disabled={bracketingApproach === BracketingApproach.None}
        style={{ marginBottom: 20 }}
        onClick={() => {
          engine.set_sub_bracket(configKey, !subBracket);
        }}
      >
        <Label>
          <p>Use sub-brackets.</p>
          <p>Bracket consecutive instruments of the same type.</p>
        </Label>
        <Switch value={subBracket} />
      </ListItem>

      <div className="generic-settings__section">
        <Subheader>Design</Subheader>
        <Select
          value={bracketStyle}
          onChange={(value: BracketStyle) => {
            engine.set_bracket_style(configKey, value);
          }}
        >
          <Option value={BracketStyle.None} displayAs="None">
            None
          </Option>
          <Option value={BracketStyle.Line} displayAs="Lines">
            Lines
          </Option>
          <Option value={BracketStyle.Wing} displayAs="Wings">
            Wings
          </Option>
        </Select>
      </div>
    </>
  );
};
