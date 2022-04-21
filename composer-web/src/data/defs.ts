import { LayoutType } from "composer-engine";

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

export interface State {
  view: View;
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
}
