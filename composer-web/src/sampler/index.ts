import { Scheduler } from "./scheduler";
import { AudioPlayer } from "./audio-player";

export * from "./types";
export const ctx = new AudioContext();
export const transport = new Scheduler(ctx);
export const player = new AudioPlayer(ctx);
