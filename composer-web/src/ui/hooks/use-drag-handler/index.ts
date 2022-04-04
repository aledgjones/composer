import { useCallback } from "react";
import { dragHandler, DragHandlerConfig } from "../../utils/drag-handler";

export function useDragHandler<T>(config: DragHandlerConfig<T>, deps: any[]) {
  return useCallback(dragHandler<T>(config), [deps]);
}
