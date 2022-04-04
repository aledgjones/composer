import { MutableRefObject, Dispatch, SetStateAction, createContext } from "react";

export interface Item {
  key: string;
  index: number;
  sorting: boolean;
  active: boolean;
  ref: MutableRefObject<HTMLDivElement | null>;
}

export interface Items {
  [key: string]: Item;
}

type ItemsUpdater = Dispatch<SetStateAction<Items>>;

interface Context {
  config: {
    direction: "x" | "y";
    onEnd: (oldIndex: number, newIndex: number) => void;
  };
  items: Items;
  setItems: ItemsUpdater;
}

export const SortableContext = createContext<Context>({
  config: { direction: "y", onEnd: () => {} },
  items: {},
  setItems: () => {},
});
