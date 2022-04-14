import { Engine } from "composer-engine";
import { Store } from "pullstate";
import { PlayerPage, State, View } from "./defs";

export const engine = new Engine();

export const ui = new Store<State>({
  view: View.Setup,
  setup: {
    dialogs: {
      players: PlayerPage.AutoNumbering,
    },
    expanded: {},
    selected: null,
    panels: {
      players: true,
      layouts: true,
    },
  },
});
