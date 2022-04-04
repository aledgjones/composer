import { useEffect } from "react";

/**
 * Hook: Set the browser theme dynamically;
 */
export function useTheme(color: string) {
  useEffect(() => {
    const metaThemeColor = document.querySelector("meta[name=theme-color]");
    if (metaThemeColor) {
      metaThemeColor.setAttribute("content", color);
    } else {
      const el = document.createElement("meta");
      el.setAttribute("name", "theme-color");
      el.setAttribute("content", color);
      document.head.appendChild(el);
    }
  }, [color]);
}
