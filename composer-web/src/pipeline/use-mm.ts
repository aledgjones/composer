import { useMemo } from "react";

export function useMM() {
  return useMemo(() => {
    const div = document.createElement("div");
    div.style.position = "fixed";
    div.style.width = "1mm";
    document.body.appendChild(div);
    const width = div.clientWidth;
    document.body.removeChild(div);
    return width;
  }, []);
}
