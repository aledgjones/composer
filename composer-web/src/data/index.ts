import { Engine } from "composer-engine";
import { Store } from "pullstate";
import { EngravePage, PlayerPage, State, View } from "./defs";

export const engine = new Engine();

export const ui = new Store<State>({
  view: View.Setup,
  setup: {
    dialogs: {
      players: { page: PlayerPage.AutoNumbering },
      engrave: { page: EngravePage.Barlines, config: null },
    },
    expanded: {},
    selected: null,
    panels: {
      players: true,
      layouts: true,
    },
  },
});
