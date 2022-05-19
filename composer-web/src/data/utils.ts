import { useEffect, useState } from "react";
import { engine, store } from ".";
import { transport } from "../sampler";

export const useFlowKey = () => {
  const flows: string[] = engine.flows;
  return store.useState(
    (s) => {
      if (s.flow && flows.includes(s.flow)) {
        return s.flow;
      } else {
        return flows[0];
      }
    },
    [flows]
  );
};

export function useTimestamp() {
  const [timestamp, setTimestamp] = useState(0);
  useEffect(() => {
    const cb = (tick: number) => setTimestamp(tick);
    transport.on("tick", cb);

    return () => {
      transport.removeListener("tick", cb);
    };
  }, []);
  return timestamp;
}

export function useSamplerSetup() {
  const flowKey = useFlowKey();
  const subdivisions = engine.get_flow_subdivisions(flowKey) || 48;
  const length = engine.get_flow_length(flowKey) || 48 * 4 * 4;

  useEffect(() => {
    transport.subdivisions = subdivisions;
    transport.length = length;
  }, [length, subdivisions]);
}

export function usePlayState() {
  const [playing, setPlaying] = useState(false);

  useEffect(() => {
    const start = () => {
      setPlaying(true);
    };

    const stop = () => {
      setPlaying(false);
    };

    transport.on("start", start);
    transport.on("stop", stop);

    return () => {
      transport.removeListener("start", start);
      transport.removeListener("stop", stop);
    };
  }, []);

  return playing;
}
