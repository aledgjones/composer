import { NoteDuration } from "composer-engine";
import { ui } from ".";
import { EngravePage, PlayerPage, SelectionType, Tool, View } from "./defs";

export const actions = {
  snap: {
    set: (value: NoteDuration) => {
      ui.update((s) => {
        s.snap = value;
      });
    },
  },
  view: {
    set: (value: View) => {
      ui.update((s) => {
        s.view = value;
      });
    },
  },
  play: {
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
  },
  setup: {
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
  },
};

(window as any).$actions = actions;
