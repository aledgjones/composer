import { useCallback, ChangeEvent, FC } from "react";
import { mdiMagnify } from "@mdi/js";

import { SearchInputProps } from "../input-base/defs";
import merge from "classnames";
import { Icon } from "../icon";

import "./styles.css";

export const Search: FC<SearchInputProps> = ({ type, value, placeholder, onChange, className, ...props }) => {
  const _onChange = useCallback((e: ChangeEvent<HTMLInputElement>) => onChange(e.target.value), [onChange]);

  return (
    <div className="ui-search__container">
      <Icon className="ui-search__icon ui-search__icon--magnify" path={mdiMagnify} size={24} />
      <input
        type="text"
        placeholder={placeholder}
        className={merge("ui-search", className)}
        value={value}
        onChange={_onChange}
        {...props}
      />
    </div>
  );
};
