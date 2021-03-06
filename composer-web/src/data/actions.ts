import { playActions } from "./actions-play";
import { appActions } from "./actions-app";
import { setupActions } from "./actions-setup";
import { uiActions } from "./actions-ui";

export const actions = {
  ui: uiActions,
  play: playActions,
  setup: setupActions,
  app: appActions,
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(window as any).$actions = actions;
