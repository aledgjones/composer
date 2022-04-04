import { FC, useState, useEffect } from "react";
import * as React from "react";

import { colors } from "./colors";
import merge from "classnames";
import { useForeground } from "../../hooks/use-foreground";

import "./styles.css";

interface Props {
  id?: string;
  className?: string;
  style?: React.CSSProperties;

  src?: string;
  name: string;
  size: number;
}

/**
 * Avatar component. Displays image else falls back to GMail style colored circle and letter.
 */
export const Avatar: FC<Props> = ({ id, className, style, src, name, size }) => {
  const letter = name.slice(0, 1).toUpperCase();
  const background = colors[letter] || "rgb(200,200,200)";
  const foreground = useForeground(background);

  const [isImageValid, setIsImageValid] = useState(false);

  useEffect(() => {
    let didCancel = false;

    async function checkImageExists(src: string) {
      setIsImageValid(false);
      try {
        await fetch(src, { mode: "no-cors" });
        if (!didCancel) {
          setIsImageValid(true);
        }
      } catch {
        // no catch as isImageValid is already false;
      }
    }

    if (src) {
      checkImageExists(src);
    }

    return () => {
      didCancel = true;
    };
  }, [src]);

  return (
    <div
      id={id}
      className={merge("ui-avatar", className)}
      style={{
        height: size,
        width: size,
        fontSize: size * 0.6,
        color: foreground,
        backgroundColor: background,
        backgroundImage: isImageValid ? `url(${src})` : undefined,
        ...style,
      }}
    >
      {!isImageValid && <span className="ui-avatar__letter">{letter}</span>}
    </div>
  );
};
