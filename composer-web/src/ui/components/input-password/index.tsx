import { FC, useCallback } from "react";

import { error } from "../../utils/error";
import { InputBase } from "../input-base";
import { PasswordInputProps } from "../input-base/defs";

export const InputPassword: FC<PasswordInputProps> = ({ value, required, ...props }) => {
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

  return <InputBase display={value} required={required} spellcheck={false} validate={validate} {...props} />;
};
