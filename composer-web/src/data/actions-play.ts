import { ui } from ".";
import { Tool } from "./defs";

export const playActions = {
  expanded: {
    toggle: (key: string) => {
      ui.update((s) => {
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
      ui.update((s) => {
        s.play.keyboard[instrumentKey] = base;
      });
    },
  },
  track: {
    set: (instrumentKey: string, staveKey: string) => {
      ui.update((s) => {
        s.play.track[instrumentKey] = staveKey;
      });
    },
  },
  tool: {
    set: (value: Tool) => {
      ui.update((s) => {
        s.play.tool = value;
      });
    },
  },
  zoom: {
    inc: () => {
      ui.update((s) => {
        const zoom = s.play.zoom;
        if (zoom + 5 <= 500) {
          s.play.zoom = zoom + 5;
        }
      });
    },
    desc: () => {
      ui.update((s) => {
        const zoom = s.play.zoom;
        if (zoom - 5 >= 25) {
          s.play.zoom = zoom - 5;
        }
      });
    },
    set: (value: number) => {
      ui.update((s) => {
        s.play.zoom = value;
      });
    },
  },
};
