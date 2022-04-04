import { useState } from "react";

import { SortableContext, Items } from "./context";
import merge from "classnames";
import { SuperFC } from "../../generic";

interface Props {
  direction: "x" | "y";
  onEnd: (oldIndex: number, newIndex: number) => void;
}

export const SortableContainer: SuperFC<HTMLDivElement, Props> = ({
  className,
  direction,
  onEnd,
  children,
  ...props
}) => {
  const [items, setItems] = useState<Items>({});

  return (
    <SortableContext.Provider value={{ config: { direction, onEnd }, items, setItems }}>
      <div className={merge("ui-sortable-container", className)} {...props}>
        {children}
      </div>
    </SortableContext.Provider>
  );
};
