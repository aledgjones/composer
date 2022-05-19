import { Tick, BPM, Ticks } from "./types";

/**
 * The Tompo Class represents a change of tempo. Either imediately or over a duration.
 */
export class Tempo {
  public at: Tick;
  public to: BPM;
  public from: BPM;
  public duration: Ticks;

  constructor(at: Tick, to: BPM, from?: BPM, duration?: Ticks) {
    this.at = at;
    this.to = to;
    this.from = from || at;
    this.duration = duration || 0;
  }

  private calculateLinearRamp(tick: Tick) {
    const x = tick - this.at;
    const grad = (this.to - this.from) / this.duration;
    const offset = this.from;

    return x * grad + offset;
  }

  /**
   * Calculate the tempo at a given tick taking into account tempo ramping between values.
   */
  public getAt(tick: Tick) {
    return tick >= this.at + this.duration
      ? this.to
      : this.calculateLinearRamp(tick);
  }
}
