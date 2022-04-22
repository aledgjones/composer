import { NoteDuration } from "composer-engine";
import { ui } from ".";
import { playActions } from "./actions-play";
import { setupActions } from "./actions-setup";
import { View } from "./defs";

export const actions = {
  snap: {
    set: (value: NoteDuration) => {
      ui.update((s) => {
        s.snap = value;
      });
    },
  },
  view: {
    set: (value: View) => {
      ui.update((s) => {
        s.view = value;
      });
    },
  },
  play: playActions,
  setup: setupActions,
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(window as any).$actions = actions;
