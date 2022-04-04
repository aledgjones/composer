import { useEffect, useState } from "react";

/**
 * Hook: listener for scroll event - returns Y scroll offset.
 */
export function useScrollListener() {
  const [y, setY] = useState(0.0);
  useEffect(() => {
    const cb = () => {
      setY(window.scrollY);
    };

    window.addEventListener("scroll", cb, { passive: true });
    return () => {
      window.removeEventListener("scroll", cb);
    };
  });

  return y;
}
