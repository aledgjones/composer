import { Store } from "pullstate";
import { SelectionType, View } from "./defs";

interface State {
  view: View;
  setup: {
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
    expanded: {},
    selected: null,
    panels: {
      players: true,
      layouts: true,
    },
  },
});
