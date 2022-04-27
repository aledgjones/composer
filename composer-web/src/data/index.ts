import {
  Engine,
  NoteDuration,
  PlayerType,
  TimeSignatureDrawType,
} from "composer-engine";
import { Store } from "pullstate";
import { EngravePage, PlayerPage, State, Tool, View } from "./defs";

export const engine = new Engine();

(window as any).engine = engine;

export const store = new Store<State>({
  selection: [],
  view: View.Setup,
  snap: NoteDuration.Sixteenth,
  audition: true,
  flow: null,
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

// TODO: remove: auto setiup score
const instruments = [
  "strings.violin",
  "strings.violin",
  "strings.viola",
  "strings.violoncello",
];
instruments.forEach((id) => {
  const playerKey = engine.create_player(PlayerType.Solo);
  const instrumentKey = engine.create_instrument(id);
  engine.assign_instrument_to_player(playerKey, instrumentKey);
});

const flow = engine.flows[0];
engine.create_time_signature(
  flow,
  0,
  3,
  NoteDuration.Quarter,
  TimeSignatureDrawType.Normal
);
