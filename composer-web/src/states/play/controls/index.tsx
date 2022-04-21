import { FC } from "react";
import { mdiChevronDown, mdiCogOutline, mdiSizeS, mdiSizeM } from "@mdi/js";
import { Text } from "../../../components/text";
import { Keyboard } from "../keyboard";
import { Fader } from "../fader";
import { SLOT_COUNT } from "../const";
import { Icon } from "../../../ui/components/icon";
import { Select } from "../../../ui/components/select";
import { Option } from "../../../ui/components/option";
import { Label } from "../../../ui/components/label";
import { noop } from "../../../ui/utils/noop";
import { engine, ui } from "../../../data";
import { actions } from "../../../data/actions";

import "./styles.css";

interface Props {
  color: string;
  instrumentKey: string;
}

export const Controls: FC<Props> = ({ color, instrumentKey }) => {
  const staves: string[] = engine.get_instrument_staves(instrumentKey);
  const expanded = ui.useState(
    (s) => s.play.expanded[instrumentKey],
    [instrumentKey]
  );
  const stave = ui.useState(
    (s) => s.play.stave[instrumentKey] || staves[0],
    [instrumentKey, staves]
  );
  const name = engine.get_instrument_name(instrumentKey);
  const id = engine.get_instrument_id(instrumentKey);
  const volume = engine.get_instrument_volume(instrumentKey);
  const solo = engine.get_instrument_solo(instrumentKey);
  const mute = engine.get_instrument_mute(instrumentKey);

  return (
    <div className="controls">
      <div className="controls__color" style={{ backgroundColor: color }} />
      <div className="controls__header">
        <p className="controls__name">
          <Text content={name} />
        </p>
        <Icon
          toggled={solo}
          style={{ marginRight: 12 }}
          path={mdiSizeS}
          size={24}
          onClick={() => {
            engine.toggle_instrument_solo(instrumentKey);
          }}
        />
        <Icon
          toggled={mute}
          path={mdiSizeM}
          size={24}
          onClick={() => {
            engine.toggle_instrument_mute(instrumentKey);
          }}
        />
        <Icon
          style={{
            marginLeft: 12,
            transform: `rotateZ(${expanded ? "180deg" : "0"})`,
          }}
          size={24}
          path={mdiChevronDown}
          onClick={() => {
            actions.play.expanded.toggle(instrumentKey);
          }}
        />
      </div>
      {expanded && (
        <>
          <div className="controls__settings-wrapper">
            <div className="controls__settings">
              <div className="controls__settings-spacer">
                <Fader
                  instrumentKey={instrumentKey}
                  volume={volume}
                  color={color}
                  onChange={(volume: number) => {
                    engine.set_instrument_volume(instrumentKey, volume);
                  }}
                />
                <Select
                  value={stave}
                  onChange={(value) => {
                    actions.play.stave.set(instrumentKey, value);
                  }}
                >
                  {staves.map((staveKey, i) => {
                    return (
                      <Option
                        key={staveKey}
                        value={staveKey}
                        displayAs={`Stave ${i + 1}`}
                      >{`Stave ${i + 1}`}</Option>
                    );
                  })}
                </Select>
              </div>
              <div className="controls__sampler-config">
                <Icon
                  style={{ marginRight: 20 }}
                  path={mdiCogOutline}
                  size={24}
                  onClick={noop}
                />
                <Label className="controls__sampler-meta">
                  <p>Solo Sampler</p>
                  <p>{id}</p>
                </Label>
              </div>
            </div>
            <Keyboard instrumentKey={instrumentKey} height={SLOT_COUNT} />
          </div>
        </>
      )}
    </div>
  );
};
