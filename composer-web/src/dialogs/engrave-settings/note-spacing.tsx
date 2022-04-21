import { FC } from "react";
import { engine } from "../../data";
import { Input } from "../../ui/components/input";
import { Subheader } from "../../ui/components/subheader";

interface Props {
  configKey: string;
}

export const NoteSpacing: FC<Props> = ({ configKey }) => {
  return (
    <div className="generic-settings__section">
      <Subheader>Note Spacing</Subheader>
      <Subheader subtle>Default space for crotchet/quarter notes</Subheader>
      <Input
        required
        type="number"
        value={engine.get_base_note_space(configKey)}
        precision={2}
        step={0.01}
        units="spaces"
        onChange={(value: number) => {
          engine.set_base_note_space(configKey, value);
        }}
      />
      <Subheader subtle>Minium space for short notes</Subheader>
      <Input
        required
        type="number"
        value={engine.get_minimum_note_space(configKey)}
        precision={2}
        step={0.01}
        units="spaces"
        onChange={(value: number) => {
          engine.set_minimum_note_space(configKey, value);
        }}
      />
      <Subheader subtle>Minium space for tied notes</Subheader>
      <Input
        required
        type="number"
        value={engine.get_minimum_tie_space(configKey)}
        precision={2}
        step={0.01}
        units="spaces"
        onChange={(value: number) => {
          engine.set_minimum_tie_space(configKey, value);
        }}
      />
      <Subheader subtle>Note space ratio</Subheader>
      <Input
        required
        type="number"
        value={engine.get_note_space_ratio(configKey)}
        precision={2}
        step={0.01}
        onChange={(value: number) => {
          engine.set_note_space_ratio(configKey, value);
        }}
      />
    </div>
  );
};
