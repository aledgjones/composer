import {
  Engine,
  NoteDuration,
  PlayerType,
  TimeSignatureDrawType,
} from "composer-engine";
import { Store } from "pullstate";
import { EngravePage, PlayerPage, State, Tool, View } from "./defs";

export const engine = new Engine();

export const ui = new Store<State>({
  view: View.Setup,
  snap: NoteDuration.Sixteenth,
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
  // "woodwinds.flute",
  // "woodwinds.oboe",
  // "woodwinds.clarinet.a",
  // "woodwinds.bassoon",
  // "keyboard.piano",
  "strings.violin",
  // "strings.violin",
  // "strings.viola",
  // "strings.violoncello",
];
instruments.forEach((id) => {
  const playerKey = engine.create_player(PlayerType.Solo);
  const instrumentKey = engine.create_instrument(id);
  engine.assign_instrument_to_player(playerKey, instrumentKey);
});

const flowKey = engine.flows[0];
engine.create_time_signature(
  flowKey,
  0,
  6,
  NoteDuration.Eighth,
  TimeSignatureDrawType.Normal
);

engine.create_time_signature(
  flowKey,
  48,
  4,
  NoteDuration.Quarter,
  TimeSignatureDrawType.Normal
);
