type Listener = (...args: any[]) => void;
interface Listeners {
  [event: string]: Set<Listener>;
}

/**
 * Super simple event emitter
 */
export class EventEmitter<T extends string> {
  private _events: Listeners = {};

  private _getEventListByName(event: T) {
    if (typeof this._events[event] === "undefined") {
      this._events[event] = new Set();
    }
    return this._events[event];
  }

  public on(event: T, fn: Listener) {
    this._getEventListByName(event).add(fn);
  }

  public emit(event: T, ...args: any[]) {
    this._getEventListByName(event).forEach((fn) => {
      fn.apply(this, args);
    });
  }

  public removeListener(event: T, fn: Listener) {
    this._getEventListByName(event).delete(fn);
  }
}
