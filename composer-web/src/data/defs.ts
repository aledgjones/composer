import {
  Articulation,
  Duration,
  NoteDuration,
  Pitch,
  Velocity,
} from "composer-engine";

export const enum Tool {
  Select,
  Draw,
  Slice,
  Erase,
}

export const enum View {
  Setup,
  Write,
  Engrave,
  Play,
  Print,
}

export const enum SelectionType {
  Player,
  Flow,
  Layout,
}

export const enum PlayerPage {
  AutoNumbering,
}

export const enum EngravePage {
  Barlines,
  BracketsAndBraces,
  NoteSpacing,
  Staves,
}

export interface Tone {
  articulation: Articulation;
  duration: Duration;
  key: string;
  pitch: Pitch;
  tick: number;
  velocity: Velocity;
}

export interface EntrySelection {
  key: string;
  trackKey: string;
}

export interface State {
  selection: EntrySelection[];
  view: View;
  snap: NoteDuration;
  audition: boolean;
  flow: string | null;
  setup: {
    dialogs: {
      players: { page: PlayerPage };
      engrave: { page: EngravePage; config: string | null };
    };
    expanded: { [key: string]: boolean };
    selected: { key: string; type: SelectionType } | null;
    panels: {
      players: boolean;
      layouts: boolean;
    };
  };
  play: {
    tool: Tool;
    zoom: number;
    expanded: { [key: string]: boolean };
    keyboard: { [key: string]: number };
    track: { [key: string]: string };
  };
}
