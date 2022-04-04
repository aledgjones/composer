import { FC, Children, useState, CSSProperties } from "react";

import merge from "classnames";
import { TabExtended } from "../tab";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: CSSProperties;
  value: any;
  onChange: (value: any) => void;
}

/**
 * Tabs component used with the Tab component.
 */
export const Tabs: FC<Props> = ({ id, style, className, value, onChange, children }) => {
  const [bar, setBar] = useState({ left: 0, width: 73 });

  return (
    <div id={id} style={style} className={merge("ui-tabs", className)}>
      {Children.map(children, (child: any) => {
        if (child) {
          return (
            <TabExtended {...child.props} selected={value === child.props.value} onChange={onChange} setBar={setBar} />
          );
        } else {
          return null;
        }
      })}
      <div className="ui-tabs__bar" style={{ ...bar }} />
    </div>
  );
};
