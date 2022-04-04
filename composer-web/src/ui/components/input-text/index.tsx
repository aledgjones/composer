import { FC, useCallback } from "react";

import { error } from "../../utils/error";
import { InputBase } from "../input-base";
import { TextInputProps } from "../input-base/defs";

export const InputText: FC<TextInputProps> = ({ value, required, ...props }) => {
  const validate = useCallback(
    (value: string) => {
      if (required && value === "") {
        return error("@ui/input-required", "Required");
      } else {
        return null;
      }
    },
    [required]
  );

  return <InputBase display={value} required={required} spellcheck={true} validate={validate} {...props} />;
};
