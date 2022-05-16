import { Engine } from "composer-engine";
import { Store } from "pullstate";
import { EngravePage, PlayerPage, State, Tool, View } from "./defs";

export const engine = new Engine();

(window as any).engine = engine;

export const store = new Store<State>({
  app: {},
  selection: [],
  view: View.Setup,
  snap: 4,
  audition: true,
  flow: null,
  zoom: 100,
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
    track: {},
  },
});
