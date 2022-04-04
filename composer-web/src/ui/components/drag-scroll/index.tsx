import { FC, CSSProperties, useState, useCallback } from "react";

import merge from "classnames";
import { useDragHandler } from "../../hooks/use-drag-handler";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;

  x?: boolean;
  y?: boolean;
  ignore?: string; // class to ignore completely
  ignoreX?: string; // class to ignoreX completely
  ignoreY?: string; // class to ignoreY completely
}

export const DragScroll: FC<Props> = ({ id, className, style, x, y, ignore, ignoreX, ignoreY, children }) => {
  const [dragging, setDragging] = useState(false);

  const allow = useCallback((target: HTMLElement, ignore?: string) => {
    if (ignore) {
      const nodes = document.getElementsByClassName(ignore);
      for (let i = 0; i < nodes.length; i++) {
        const node = nodes.item(i);
        if (node && (node === target || node.contains(target))) {
          return false;
        }
      }
      return true;
    } else {
      return true;
    }
  }, []);

  const onDrag = useDragHandler<{
    target: HTMLElement;
    allowX: boolean;
    allowY: boolean;
    x: number;
    y: number;
    scrollLeft: number;
    scrollTop: number;
  }>(
    {
      onDown: (e) => {
        // only accept unmodified left mouse buttons
        if (e.button > 0 || e.ctrlKey || e.shiftKey || e.metaKey || !allow(e.target as HTMLElement, ignore)) {
          return false;
        }

        const allowX = allow(e.target as HTMLElement, ignoreX);
        const allowY = allow(e.target as HTMLElement, ignoreY);

        setDragging(true);

        return {
          target: e.currentTarget as HTMLElement,
          allowX,
          allowY,
          x: e.screenX,
          y: e.screenY,
          scrollLeft: e.currentTarget.scrollLeft,
          scrollTop: e.currentTarget.scrollTop,
        };
      },
      onMove: (e, init) => {
        if (x && init.allowX) {
          const diff = e.screenX - init.x;
          const scrollLeft = init.scrollLeft - diff;
          const min = 0;
          const max = init.target.scrollWidth - init.target.offsetWidth;
          if (scrollLeft < min) {
            // if you over scroll, offset the init position so we start scrolling again
            // straight away on direction change
            init.x = init.x - scrollLeft;
          } else if (scrollLeft > max) {
            init.x = init.x - (scrollLeft - max);
          }
          init.target.scrollLeft = scrollLeft; // this is always bound so just keep setting it.
        }
        if (y && init.allowY) {
          const diff = e.screenY - init.y;
          const scrollTop = init.scrollTop - diff;
          const min = 0;
          const max = init.target.scrollHeight - init.target.offsetHeight;
          if (scrollTop < min) {
            // if you over scroll, offset the init position so we start scrolling again
            // straight away on direction change
            init.y = init.y - scrollTop;
          } else if (scrollTop > max) {
            init.y = init.y - (scrollTop - max);
          }
          init.target.scrollTop = scrollTop;
        }
      },
      onEnd: () => {
        setDragging(false);
      },
    },
    [x, y, allow, ignore, ignoreX, ignoreY]
  );

  return (
    <div
      onPointerDown={onDrag}
      id={id}
      className={merge(
        "ui-drag-scroll",
        {
          "ui-drag-scroll-x": x,
          "ui-drag-scroll-y": y,
          "ui-drag-scroll--dragging": dragging,
        },
        className
      )}
      style={style}
    >
      {children}
    </div>
  );
};
