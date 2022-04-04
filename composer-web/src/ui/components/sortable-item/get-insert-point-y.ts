import { Item } from "../sortable-container/context";

export function getInsertPointY(e: PointerEvent, items: Item[], oldIndex: number) {
  let newIndex = 0;
  items.forEach((item) => {
    const box = item.ref.current?.getBoundingClientRect();
    if (box) {
      const insertCutOffPoint = box.top + box.height / 2;
      // don't insertAt the existing index
      if (e.clientY > insertCutOffPoint) {
        if (item.index > oldIndex) {
          if (item.index > newIndex) {
            newIndex = item.index;
          }
        } else if (item.index < oldIndex) {
          if (item.index + 1 > newIndex) {
            newIndex = item.index + 1;
          }
        }
      }
    }
  });
  return newIndex;
}
