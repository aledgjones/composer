import { FC } from "react";
import { engine } from "../../data";
import { Input } from "../../ui/components/input";
import { Subheader } from "../../ui/components/subheader";

import staveSpace from "./examples/stave-space.svg";

interface Props {
  configKey: string;
}

export const Staves: FC<Props> = ({ configKey }) => {
  return (
    <div className="generic-settings__section">
      <Subheader>Space Size</Subheader>
      <div className="generic-settings__input-with-img">
        <img
          alt="Stave spacing"
          src={staveSpace}
          className="generic-settings__example"
          width="95"
        />
        <Input
          required
          type="number"
          value={engine.get_space(configKey)}
          precision={2}
          step={0.01}
          units="mm"
          onChange={(value: number) => {
            engine.set_space(configKey, value);
          }}
        />
      </div>
    </div>
  );
};
