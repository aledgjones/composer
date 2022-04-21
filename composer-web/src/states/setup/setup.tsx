import { PlayerType } from "composer-engine";
import { FC, useState } from "react";
import { RenderRegion } from "../../components/render-region";
import { Renderer } from "../../components/renderer";
import { InstrumentPicker } from "../../dialogs/instrument-picker";
import { PlayerTypePicker } from "../../dialogs/player-type-picker";
import { engine, ui } from "../../data";
import { SelectionType } from "../../data/defs";

import { useTitle } from "../../ui/hooks/use-title";
import { FlowList } from "./flow-list";
import { LayoutList } from "./layout-list";
import { PlayerList } from "./player-list";
import { actions } from "../../data/actions";

import "./setup.css";

const Setup: FC = () => {
  useTitle("Solo Composer | Setup");

  const [playerKey, setPlayerKey] = useState<string | null>(null);
  const [showPlayerPicker, setShowPlayerPicker] = useState(false);
  const [showInstrumentPicker, setShowInstrumentPicker] = useState(false);

  const pickPlayerType = () => {
    setShowPlayerPicker(true);
  };

  const onAddInstrument = (key: string) => {
    setPlayerKey(key);
    setShowInstrumentPicker(true);
  };

  const onPlayerTypePicked = (type: PlayerType) => {
    setShowPlayerPicker(false);
    const playerKey = engine.create_player(type);
    actions.setup.selection.set(playerKey, SelectionType.Player);
    setPlayerKey(playerKey);
    setShowInstrumentPicker(true);
  };

  const onInstrumentPicked = (def: string) => {
    setShowInstrumentPicker(false);
    const instrumentKey = engine.create_instrument(def);
    engine.assign_instrument_to_player(playerKey as string, instrumentKey);
  };

  return (
    <>
      <div className="setup">
        <PlayerList
          onCreatePlayer={pickPlayerType}
          onAddInstrument={onAddInstrument}
          onSelect={actions.setup.selection.set}
          onClear={actions.setup.selection.clear}
        />

        <div className="setup__middle">
          <RenderRegion className="setup__view">
            {engine.flows.map((flowKey: string) => (
              <Renderer key={flowKey} flowKey={flowKey} />
            ))}
          </RenderRegion>
          <FlowList
            onSelect={actions.setup.selection.set}
            onClear={actions.setup.selection.clear}
          />
        </div>

        <LayoutList />
      </div>

      <PlayerTypePicker
        width={400}
        open={showPlayerPicker}
        onSelect={onPlayerTypePicked}
        onCancel={() => setShowPlayerPicker(false)}
      />

      <InstrumentPicker
        width={900}
        open={showInstrumentPicker}
        onSelect={onInstrumentPicked}
        onCancel={() => setShowInstrumentPicker(false)}
      />
    </>
  );
};

export default Setup;
