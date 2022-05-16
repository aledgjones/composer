import { mdiMinus, mdiPlus } from "@mdi/js";
import { FC } from "react";
import { Icon } from "../../ui/components/icon";
import { Option } from "../../ui/components/option";
import { Select } from "../../ui/components/select";

import "./styles.css";

interface Props {
  zoom: number;
  inc: () => void;
  set: (value: number) => void;
  desc: () => void;
}

export const Zoom: FC<Props> = ({ zoom, inc, set, desc }) => {
  return (
    <div className="zoom">
      <Icon className="zoom__icon" path={mdiMinus} size={22} onClick={desc} />
      <Select
        className="zoom__select"
        direction="up"
        value={zoom}
        onChange={set}
      >
        {/* This is a bit weired but we need a fake option to hold the current,
                possibly abartrary, zoom level. It is hidden with CSS */}
        <Option value={zoom} displayAs={`${zoom}%`} />
        <Option value={25} displayAs="25%">
          25%
        </Option>
        <Option value={50} displayAs="50%">
          50%
        </Option>
        <Option value={75} displayAs="75%">
          75%
        </Option>
        <Option value={100} displayAs="100%">
          100%
        </Option>
        <Option value={150} displayAs="150%">
          150%
        </Option>
        <Option value={200} displayAs="200%">
          200%
        </Option>
        <Option value={300} displayAs="300%">
          300%
        </Option>
        <Option value={400} displayAs="400%">
          400%
        </Option>
        <Option value={500} displayAs="500%">
          500%
        </Option>
      </Select>
      <Icon className="zoom__icon" path={mdiPlus} size={22} onClick={inc} />
    </div>
  );
};
