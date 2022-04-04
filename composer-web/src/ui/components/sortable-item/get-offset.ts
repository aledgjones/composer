import { Item } from "../sortable-container/context";

export function getOffset(item: Item, insertAt: number, index: number, offsetItemsBy: number) {
  if (item.index < index) {
    if (insertAt <= item.index) {
      return offsetItemsBy;
    }
  }
  if (item.index > index) {
    if (insertAt >= item.index) {
      return -offsetItemsBy;
    }
  }
  return 0;
}
