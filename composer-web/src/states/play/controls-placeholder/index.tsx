import { FC } from "react";
import { Text } from "../../../components/text";
import { engine } from "../../../data";

import "./styles.css";

interface Props {
  color: string;
  instrumentKey: string;
}

export const ControlsPlaceholder: FC<Props> = ({ color, instrumentKey }) => {
  const name = engine.get_instrument_name(instrumentKey);

  return (
    <div className="controls-placeholder">
      <div
        className="controls-placeholder__color"
        style={{ backgroundColor: color }}
      />
      <div className="controls-placeholder__header">
        <p className="controls-placeholder__name">
          <Text content={name} />
        </p>
      </div>
    </div>
  );
};
