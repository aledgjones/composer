import { FC, useCallback, useEffect, useState } from "react";
import pretty from "pretty-ms";

interface Props {
  when: number;
}

export const Duration: FC<Props> = ({ when }) => {
  const cb = useCallback(() => {
    const calc = Date.now() - when;
    if (calc < 30 * 1000) {
      return "just now";
    } else {
      const str = pretty(calc, {
        compact: true,
        separateMilliseconds: true,
        unitCount: 1,
        verbose: true,
      });
      return str + " ago";
    }
  }, [when]);

  const [display, setDisplay] = useState(cb());

  useEffect(() => {
    const interval = setInterval(() => {
      setDisplay(cb());
    }, 1000);
    return () => {
      clearInterval(interval);
    };
  }, [cb]);

  return <span>{display}</span>;
};
