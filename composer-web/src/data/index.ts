import { Engine, NoteDuration } from "composer-engine";
import { Store } from "pullstate";
import { EngravePage, PlayerPage, State, Tool, View } from "./defs";

export const engine = new Engine();

export const ui = new Store<State>({
  view: View.Setup,
  snap: NoteDuration.Sixteenth,
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
  play: {
    tool: Tool.Select,
    zoom: 100,
    expanded: {},
    keyboard: {},
    stave: {},
  },
});
