import { PlayerType } from "composer-engine";
import { FC, useState } from "react";
import { RenderRegion } from "../../components/render-region";
import { Renderer } from "../../components/renderer";
import { InstrumentPicker } from "../../dialogs/instrument-picker";
import { PlayerTypePicker } from "../../dialogs/player-type-picker";
import { engine, store } from "../../data";
import { SelectionType } from "../../data/defs";
import { useTitle } from "../../ui/hooks/use-title";
import { FlowList } from "./flow-list";
import { LayoutList } from "./layout-list";
import { PlayerList } from "./player-list";
import { actions } from "../../data/actions";
import { BottomBar } from "../../components/bottom-bar";
import { Zoom } from "../../components/zoom";

import "./setup.css";

const Setup: FC = () => {
  useTitle("Solo Composer | Setup");

  const [playerKey, setPlayerKey] = useState<string | null>(null);
  const [showPlayerPicker, setShowPlayerPicker] = useState(false);
  const [showInstrumentPicker, setShowInstrumentPicker] = useState(false);

  const zoom = store.useState((s) => s.zoom);

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

  const onSelect = (x: number, y: number) => {
    console.log(`${x},${y}`);
  };

  return (
    <>
      <div className="setup">
        <div className="setup__content">
          <PlayerList
            onCreatePlayer={pickPlayerType}
            onAddInstrument={onAddInstrument}
            onSelect={actions.setup.selection.set}
            onClear={actions.setup.selection.clear}
          />

          <div className="setup__middle">
            <RenderRegion className="setup__view">
              {engine.flows.map((flowKey: string) => (
                <Renderer onSelect={onSelect} key={flowKey} flowKey={flowKey} />
              ))}
            </RenderRegion>
            <FlowList
              onSelect={actions.setup.selection.set}
              onClear={actions.setup.selection.clear}
            />
          </div>

          <LayoutList />
        </div>
        <BottomBar>
          <div />
          <Zoom
            zoom={zoom}
            inc={actions.ui.zoom.inc}
            set={actions.ui.zoom.set}
            desc={actions.ui.zoom.desc}
          />
        </BottomBar>
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
