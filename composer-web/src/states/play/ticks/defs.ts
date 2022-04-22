export interface Tick {
  x: number;
  width: number;
  first: boolean;
  beat: boolean;
  sub_beat: boolean;
  boundry: boolean;
}

export interface TickList {
  list: Tick[];
  width: number;
}
