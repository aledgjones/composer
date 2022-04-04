import { FC, useCallback } from "react";

import { error } from "../../utils/error";
import { isEmail } from "../../utils/is-email";
import { InputBase } from "../input-base";
import { EmailInputProps } from "../input-base/defs";

export const InputEmail: FC<EmailInputProps> = ({ value, required, ...props }) => {
  const validate = useCallback(
    (value: string) => {
      if (required && value === "") {
        return error("@ui/input-required", "Required");
      } else if (!isEmail(value)) {
        return error("@ui/input-email-invalid", "Email invalid");
      } else {
        return null;
      }
    },
    [required]
  );

  return <InputBase display={value} required={required} spellcheck={false} validate={validate} {...props} />;
};
