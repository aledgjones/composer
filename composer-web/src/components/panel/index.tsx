import { FC } from "react";
import { mdiChevronLeft, mdiChevronRight } from "@mdi/js";
import merge from "classnames";

import { Icon } from "../../ui/components/icon";

import "./styles.css";

export const enum CollpaseDirection {
  None,
  Left,
  Right,
}

interface Props {
  collapsed?: boolean;
  collapse?: CollpaseDirection;
  onToggle?: () => void;
  className: string;
}

export const Panel: FC<Props> = ({
  children,
  className,
  collapse,
  collapsed,
  onToggle,
}) => {
  return (
    <div className={merge("panel", className)}>
      {collapse === CollpaseDirection.Left && (
        <div
          className="panel__collapse panel__collapse--left"
          onClick={onToggle}
        >
          <Icon
            style={{
              transform: `rotateZ(${collapsed ? 0 : 180}deg)`,
            }}
            path={mdiChevronLeft}
            size={12}
          />
        </div>
      )}
      {!collapsed && <div className="panel__content">{children}</div>}
      {collapse === CollpaseDirection.Right && (
        <div
          className="panel__collapse panel__collapse--left"
          onClick={onToggle}
        >
          <Icon
            style={{
              transform: `rotateZ(${collapsed ? 0 : 180}deg)`,
            }}
            path={mdiChevronRight}
            size={12}
          />
        </div>
      )}
    </div>
  );
};
