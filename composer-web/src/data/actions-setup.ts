import { ui } from ".";
import { EngravePage, PlayerPage, SelectionType } from "./defs";

export const setupActions = {
  expanded: {
    toggle: (key: string) => {
      ui.update((s) => {
        const isExpanded = s.setup.expanded[key];
        if (isExpanded) {
          delete s.setup.expanded[key];
        } else {
          s.setup.expanded[key] = true;
        }
      });
    },
  },
  panels: {
    players: {
      toggle: () => {
        ui.update((s) => {
          s.setup.panels.players = !s.setup.panels.players;
        });
      },
    },
    layout: {
      toggle: () => {
        ui.update((s) => {
          s.setup.panels.layouts = !s.setup.panels.layouts;
        });
      },
    },
  },
  selection: {
    set: (key: string, type: SelectionType) => {
      ui.update((s) => {
        s.setup.selected = { key, type };
      });
    },
    clear: () => {
      ui.update((s) => {
        s.setup.selected = null;
      });
    },
  },
  dialogs: {
    players: {
      page: {
        set: (page: PlayerPage) => {
          ui.update((s) => {
            s.setup.dialogs.players.page = page;
          });
        },
      },
    },
    engrave: {
      config: {
        set: (key: string) => {
          ui.update((s) => {
            s.setup.dialogs.engrave.config = key;
          });
        },
      },
      page: {
        set: (page: EngravePage) => {
          ui.update((s) => {
            s.setup.dialogs.engrave.page = page;
          });
        },
      },
    },
  },
};
