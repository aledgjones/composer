import { PlayerType } from "composer-engine";
import { FC, useState } from "react";
import { InstrumentPicker } from "../../dialogs/instrument-picker";
import { PlayerTypePicker } from "../../dialogs/player-type-picker";
import { engine } from "../../engine";

import { useTitle } from "../../ui/hooks/use-title";
import { PlayerList } from "./player-list";

import "./setup.css";

const Setup: FC = () => {
  useTitle("Solo Composer | Setup");

  const [playerKey, setPlayerKey] = useState(null);
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
    const playerKey = engine.add_player(type);
    setPlayerKey(playerKey);
    setShowInstrumentPicker(true);
  };

  const onInstrumentPicked = (def: string) => {
    setShowInstrumentPicker(false);
    const instrumentKey = engine.create_instrument(def);
    engine.assign_instrument(playerKey, instrumentKey);
  };

  return (
    <>
      <div className="setup">
        <PlayerList
          onCreatePlayer={pickPlayerType}
          onAddInstrument={onAddInstrument}
        />
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
