import { store } from ".";
import { EntrySelection, View } from "./defs";

export const uiActions = {
  selection: {
    select: (entry: EntrySelection) => {
      store.update((s) => {
        const found = Boolean(
          s.selection.find((item) => item.key === entry.key)
        );
        if (!found) {
          s.selection.push(entry);
        }
      });
    },
    deselect: (key: string) => {
      store.update((s) => {
        s.selection = s.selection.filter((item) => item.key !== key);
      });
    },
    clear: () => {
      store.update((s) => {
        s.selection = [];
      });
    },
  },
  snap: {
    set: (value: number) => {
      store.update((s) => {
        s.snap = value;
      });
    },
  },
  view: {
    set: (value: View) => {
      store.update((s) => {
        s.view = value;
      });
    },
  },
};
