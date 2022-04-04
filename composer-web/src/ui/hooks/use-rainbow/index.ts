import { useMemo } from "react";
/**
 * Hook: Generates a array of colors of length 'count'. Minimum number of colors is 7.
 */
export function useRainbow(count: number) {
  return useMemo(() => {
    const len = count > 7 ? count : 7;

    const offset = 0;
    const width = 359 - 100;
    const step = Math.floor(width / len);

    return Array(len)
      .fill("")
      .map((_entry, i) => {
        const base = step * i + offset;
        const color = `hsl(${base > 359 ? base - 359 : base}, 85%, 55%)`;
        return color;
      });
  }, [count]);
}
