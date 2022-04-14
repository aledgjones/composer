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

export interface State {
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
