import { FC, CSSProperties, Children, useState, useRef, useEffect } from "react";
import merge from "classnames";
import { Card } from "../card";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
}

export const MenuBar: FC<Props> = ({ id, className, style, children }) => {
  // is the menu bar currently active?
  const [open, setOpen] = useState(false);
  // which item is currently selected
  const [selection, setSelection] = useState("");
  const element = useRef<HTMLDivElement>(null);

  // auto close
  useEffect(() => {
    const cb = (e: any) => {
      if (!element.current || !element.current.contains(e.target)) {
        setOpen(false);
      } else {
        setOpen((o) => !o);
      }
    };
    document.addEventListener("click", cb);
    return () => document.removeEventListener("click", cb);
  }, [element]);

  return (
    <div id={id} className={merge("ui-menu-bar", className)} style={style}>
      <div ref={element} className="ui-menu-bar__content">
        {Children.map(children, (child: any) => {
          if (child) {
            const selected = open && child.props.label === selection;
            return (
              <div
                id={id}
                className={merge("ui-menu-bar-item", { "ui-menu-bar-item--selected": selected }, className)}
                style={style}
                onPointerEnter={() => setSelection(child.props.label)}
              >
                {child.props.label}
                {selected && <Card className="ui-menu-bar-item__card">{child.props.children}</Card>}
              </div>
            );
          } else {
            return null;
          }
        })}
      </div>
    </div>
  );
};
