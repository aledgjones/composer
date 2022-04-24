import { store } from ".";
import { Tool } from "./defs";

export const playActions = {
  expanded: {
    toggle: (key: string) => {
      store.update((s) => {
        const isExpanded = s.play.expanded[key];
        if (isExpanded) {
          delete s.play.expanded[key];
        } else {
          s.play.expanded[key] = true;
        }
      });
    },
  },
  keyboard: {
    set: (instrumentKey: string, base: number) => {
      store.update((s) => {
        s.play.keyboard[instrumentKey] = base;
      });
    },
  },
  track: {
    set: (instrumentKey: string, trackKey: string) => {
      store.update((s) => {
        s.play.track[instrumentKey] = trackKey;
      });
    },
  },
  tool: {
    set: (value: Tool) => {
      store.update((s) => {
        s.play.tool = value;
      });
    },
  },
  zoom: {
    inc: () => {
      store.update((s) => {
        const zoom = s.play.zoom;
        if (zoom + 5 <= 500) {
          s.play.zoom = zoom + 5;
        }
      });
    },
    desc: () => {
      store.update((s) => {
        const zoom = s.play.zoom;
        if (zoom - 5 >= 25) {
          s.play.zoom = zoom - 5;
        }
      });
    },
    set: (value: number) => {
      store.update((s) => {
        s.play.zoom = value;
      });
    },
  },
};
