type DownCallback<T> = (e: React.PointerEvent) => T | false;
type OtherCallback<T> = (e: PointerEvent, data: T) => void;

export interface DragHandlerConfig<T> {
  onDown: DownCallback<T>;
  onMove: OtherCallback<T>;
  onEnd: OtherCallback<T>;
}

/**
 * Boilerplate for adding pointer move events after an initial pointer down event
 */
export function dragHandler<T>({ onDown, onMove, onEnd }: DragHandlerConfig<T>) {
  let pointer: number | undefined = undefined;

  return (e: React.PointerEvent<HTMLElement>) => {
    // only one pointer at a time
    if (pointer !== undefined) return;

    const data = onDown(e);

    if (data !== false) {
      pointer = e.pointerId;

      const move = (ev: PointerEvent) => {
        if (pointer !== ev.pointerId) return;

        onMove(ev, data);
        return false;
      };

      const stop = (ev: PointerEvent) => {
        if (pointer !== ev.pointerId) return;

        onEnd(ev, data);

        document.removeEventListener("pointermove", move);
        document.removeEventListener("pointerup", stop);
        document.removeEventListener("pointercancel", stop);

        pointer = undefined;
      };

      document.addEventListener("pointermove", move, { passive: true });
      document.addEventListener("pointerup", stop, { passive: true });
      document.addEventListener("pointercancel", stop, { passive: true });
    }
  };
}
