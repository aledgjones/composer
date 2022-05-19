import { FC } from "react";
import { mdiPlay, mdiMetronome, mdiSkipPrevious, mdiPause } from "@mdi/js";
import { Icon } from "../../ui/components/icon";
import { noop } from "../../ui/utils/noop";
import { player, transport } from "../../sampler";
import {
  useFlowKey,
  usePlayState,
  useSamplerSetup,
  useTimestamp,
} from "../../data/utils";

import "./styles.css";

export const Transport: FC = () => {
  const flowKey = useFlowKey();
  const timestamp = useTimestamp(flowKey);
  const playing = usePlayState();
  useSamplerSetup(flowKey);

  return (
    <div className="transport">
      <div className="transport__controls">
        <Icon
          onClick={() => {
            player.stopAll();
            transport.seek(0);
          }}
          className="transport__icon"
          size={24}
          path={mdiSkipPrevious}
        />
        <Icon
          size={24}
          path={playing ? mdiPause : mdiPlay}
          toggled={playing}
          onClick={() => {
            if (playing) {
              transport.pause();
              player.stopAll();
            } else {
              transport.start();
            }
          }}
        />
      </div>
      <div className="transport__timestamp">
        <span>{timestamp}</span>
      </div>
      <div className="transport__metronome">
        <Icon toggled={false} onClick={noop} size={24} path={mdiMetronome} />
      </div>
    </div>
  );
};
