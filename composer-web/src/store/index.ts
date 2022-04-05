import { Store } from "pullstate";
import { PlayerPage, SelectionType, View } from "./defs";

interface State {
  view: View;
  setup: {
    dialogs: {
      players: PlayerPage;
    };
    expanded: { [key: string]: boolean };
    selected?: { key: string; type: SelectionType };
    panels: {
      players: boolean;
      layouts: boolean;
    };
  };
}

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
