import { useRef, useEffect, useContext, MutableRefObject, useLayoutEffect } from "react";
import { nanoid } from "nanoid";

import { SortableContext, Items } from "../sortable-container/context";
import { useDragHandler } from "../../hooks/use-drag-handler";
import merge from "classnames";
import { SuperFC } from "../../generic";
import { getAbsoluteHeight } from "./get-absolute-height";
import { getAbsoluteWidth } from "./get-absolute-width";
import { getInsertPointX } from "./get-insert-point-x";
import { getInsertPointY } from "./get-insert-point-y";
import { getOffset } from "./get-offset";

interface Props {
  index: number;
  handle?: MutableRefObject<HTMLElement | null>;
}

export const SortableItem: SuperFC<HTMLDivElement, Props> = ({
  index,
  handle,
  className,
  onPointerDown,
  children,
  ...props
}) => {
  const ref = useRef<HTMLDivElement>(null);

  // set a fixed key for the duration of the items life
  const key = useRef(nanoid());
  const { config, items, setItems } = useContext(SortableContext);

  // if exists, the index has changed so update else register with the container
  useLayoutEffect(() => {
    ref.current?.style.removeProperty("transform");
    setItems((items) => {
      return {
        ...items,
        [key.current]: { key: key.current, index, sorting: false, active: false, ref },
      };
    });
  }, [key, index, ref, setItems]);

  // cleanup on item destroyed
  useEffect(() => {
    return () => {
      setItems((items) => {
        const { [key.current]: item, ...others } = items;
        return others;
      });
    };
  }, [key]);

  // stop native touch scrolling in the config.direction
  useEffect(() => {
    const target = handle && handle.current ? handle.current : ref.current;
    if (target) {
      target.style.touchAction = `pan-${config.direction === "x" ? "y" : "x"}`;
    }
  }, [handle, ref, config]);

  const onDown = useDragHandler<{ x: number; y: number; offsetItemsBy: number; moveTo: number }>(
    {
      onDown: (e) => {
        if (onPointerDown) {
          onPointerDown(e as any);
        }

        // cancel if not the left mouse button
        if (e.button === 2) {
          return false;
        }

        // cancel any pointerdown events not inside the handle
        if (handle && handle.current) {
          const target = e.target as HTMLDivElement;
          if (handle.current !== e.target && !handle.current.contains(target)) {
            return false;
          }
        }

        setItems((items) => {
          return Object.entries(items).reduce<Items>((output, [itemKey, item]) => {
            return {
              ...output,
              [itemKey]: {
                ...item,
                active: itemKey === key.current,
                sorting: true,
              },
            };
          }, {});
        });

        // init mouse/pointer position
        return {
          x: e.screenX,
          y: e.screenY,
          offsetItemsBy: config.direction === "x" ? getAbsoluteWidth(ref.current) : getAbsoluteHeight(ref.current),
          moveTo: index,
        };
      },
      onMove: (e, init) => {
        if (config.direction === "x") {
          init.moveTo = getInsertPointX(e, Object.values(items), index);
          Object.entries(items).forEach(([itemKey, item]) => {
            item.ref.current?.style.setProperty(
              "transform",
              `translate3d(${
                itemKey === key.current ? e.screenX - init.x : getOffset(item, init.moveTo, index, init.offsetItemsBy)
              }px, 0px, 0px)`
            );
          });
        } else {
          init.moveTo = getInsertPointY(e, Object.values(items), index);
          Object.entries(items).forEach(([itemKey, item]) => {
            item.ref.current?.style.setProperty(
              "transform",
              `translate3d(0px, ${
                itemKey === key.current ? e.screenY - init.y : getOffset(item, init.moveTo, index, init.offsetItemsBy)
              }px, 0px)`
            );
          });
        }
      },
      onEnd: (_e, init) => {
        // if no move
        if (init.moveTo === index) {
          Object.values(items).forEach((item) => {
            item.ref.current?.style.removeProperty("transform");
          });
          setItems((items) => {
            return Object.entries(items).reduce<Items>((output, [itemKey, item]) => {
              return {
                ...output,
                [itemKey]: {
                  ...item,
                  active: false,
                  sorting: false,
                },
              };
            }, {});
          });
        } else {
          config.onEnd(index, init.moveTo);
        }
      },
    },
    [key, config, items, index, handle, onPointerDown]
  );

  const item = items[key.current];

  return (
    <div
      ref={ref}
      onPointerDown={onDown}
      className={merge(
        "ui-sortable-item",
        {
          "ui-sortable-item--sorting": item?.sorting,
          "ui-sortable-item--active": item?.active,
        },
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};
