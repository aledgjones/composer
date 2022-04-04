import { useState, useEffect } from "react";

/**
 * Hook: Delay a boolean change by a fixed time (used to delay an overlay destruction after hidden)
 */
export function useDelayBoolean(master: boolean, delay: number) {
  const [slave, setSlave] = useState(false);

  useEffect(() => {
    let timeout: any;
    if (master) {
      setSlave(true);
    } else {
      timeout = setTimeout(() => setSlave(false), delay);
    }
    return () => {
      if (timeout) {
        clearTimeout(timeout);
      }
    };
  }, [master, delay]);

  return slave;
}
