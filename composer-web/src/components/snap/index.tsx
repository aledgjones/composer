import { NoteDuration } from "composer-engine";
import { FC } from "react";
import { ui } from "../../data";
import { actions } from "../../data/actions";
import { Option } from "../../ui/components/option";
import { Select } from "../../ui/components/select";

import "./styles.css";

export const Snap: FC = () => {
  const snapDuration = ui.useState((s) => s.snap);

  return (
    <div className="snap">
      <Select
        className="snap__select"
        direction="up"
        value={snapDuration}
        onChange={actions.snap.set}
      >
        <Option value={NoteDuration.Eighth} displayAs={"\u{E1D7}"}>
          {"\u{E1D7}"}
        </Option>
        <Option value={NoteDuration.Sixteenth} displayAs={"\u{E1D9}"}>
          {"\u{E1D9}"}
        </Option>
        <Option value={NoteDuration.ThirtySecond} displayAs={"\u{E1DB}"}>
          {"\u{E1DB}"}
        </Option>
      </Select>
    </div>
  );
};
