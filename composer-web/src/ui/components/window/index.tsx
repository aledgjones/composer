import { useState, useEffect, FC } from "react";
import { createPortal } from "react-dom";

interface Props {
  styles: string;
}

/**
 * Portal in new window.
 */
export const Window: FC<Props> = ({ children, styles }) => {
  const [root, setRoot] = useState<HTMLElement>(null);

  // init
  useEffect(() => {
    const view = window.open("", "ui-console", "menubar=no,toolbar=no,location=no,titlebar=no,status=no");
    if (view) {
      view.document.body.innerHTML = "";
      const element = view.document.createElement("div");
      element.id = "ui-console--root";
      view.document.body.append(element);
      const style = view.document.createElement("style");
      style.textContent = styles;
      view.document.body.append(style);
      setRoot(element);
      return () => {
        view.close();
      };
    }
  }, []);

  if (root) {
    return createPortal(children, root);
  } else {
    return null;
  }
};
