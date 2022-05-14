export enum Idx {
  X,
  Width,
  First,
  Beat,
  SubBeat,
  Boundery,
}
export type Tick = [number, number, boolean, boolean, boolean, boolean];

export interface TickList {
  list: Tick[];
  width: number;
}
