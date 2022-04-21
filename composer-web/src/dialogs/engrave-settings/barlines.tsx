import { FC } from "react";
import { engine } from "../../data";
import { Label } from "../../ui/components/label";
import { ListItem } from "../../ui/components/list-item";
import { Subheader } from "../../ui/components/subheader";
import { Switch } from "../../ui/components/switch";

interface Props {
  configKey: string;
}

export const Barlines: FC<Props> = ({ configKey }) => {
  const systemicBarlineSingleInstrumentSystem =
    engine.get_systemic_barline_single_instrument_system(configKey);
  return (
    <>
      <div className="generic-settings__section">
        <Subheader style={{ marginBottom: 0 }}>Systemic Barlines</Subheader>
      </div>
      <ListItem
        style={{ marginBottom: 20 }}
        onClick={() => {
          engine.set_systemic_barline_single_instrument_system(
            configKey,
            !systemicBarlineSingleInstrumentSystem
          );
        }}
      >
        <Label>
          <p>Use systemic barlines for single stave systems.</p>
          <p>
            Systemic barlines will always be used with multiple instruments.
          </p>
        </Label>
        <Switch value={systemicBarlineSingleInstrumentSystem} />
      </ListItem>
    </>
  );
};
