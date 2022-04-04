import { useEffect, useState } from "react";
/**
 * Hook: Listens for window resize and returns width and height;
 */
export function useWindowResizeListener() {
  const [width, setWidth] = useState(window.innerWidth);
  const [height, setHeight] = useState(window.innerHeight);

  useEffect(() => {
    const cb = () => {
      setWidth(window.innerWidth);
      setHeight(window.innerHeight);
    };

    window.addEventListener("resize", cb, { passive: true });
    return () => {
      window.removeEventListener("resize", cb);
    };
  });

  return { width, height };
}
