import { FC, useCallback, useState } from "react";
import { mdiCheck, mdiTagOutline } from "@mdi/js";
import merge from "classnames";

import { copy } from "../../ui/utils/copy";
import { Icon } from "../../ui/components/icon";

import "./styles.css";

interface Props {
  content: string;
}

export const TagCopier: FC<Props> = ({ content }) => {
  const [working, setWorking] = useState(false);
  const trigger = useCallback(() => {
    copy(content);
    setWorking(true);
    setTimeout(() => setWorking(false), 1000);
  }, [content]);

  return (
    <span
      className={merge("tag-copier", { "tag-copier--working": working })}
      data-tooltip={content}
      data-tooltip-direction="down"
    >
      <Icon
        size={16}
        path={working ? mdiCheck : mdiTagOutline}
        onClick={trigger}
      />
    </span>
  );
};
