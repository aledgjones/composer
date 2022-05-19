import { InstrumentPlayer } from "./instrument-player";

export class AudioPlayer {
  public ctx: AudioContext;
  private instruments: Map<string, InstrumentPlayer> = new Map();

  constructor(ctx: AudioContext) {
    this.ctx = ctx;
  }

  /**
   * Deal with muting and unmuting the MuteNode depending on overall state.
   *
   * Muting is dependant on solo instruments so it must take into account the whole state
   */
  public setSamplerMuteStates() {
    let soloFound = false;
    for (const instrument of this.instruments.values()) {
      if (instrument.solo) {
        soloFound = true;
        break;
      }
    }

    for (const instrument of this.instruments.values()) {
      if (instrument.solo) {
        instrument.muteNode.gain.value = 1;
      } else if (soloFound || instrument.mute) {
        instrument.muteNode.gain.value = 0;
      } else {
        instrument.muteNode.gain.value = 1;
      }
    }
  }

  public RMS(key: string) {
    return this.instruments.get(key)?.RMS() || 0;
  }

  public peak(key: string) {
    return this.instruments.get(key)?.peak() || 0;
  }

  /**
   * Create a new sampler controled by the audio player
   */
  public createSampler(key: string): InstrumentPlayer {
    const sampler = new InstrumentPlayer(this.ctx);
    this.instruments.set(key, sampler);
    this.setSamplerMuteStates();
    return sampler;
  }

  /**
   * Play a note
   */
  public play(
    instrument: string,
    expression: number,
    pitch: number,
    when: number,
    duration: number
  ) {
    this.instruments.get(instrument)?.play(expression, pitch, when, duration);
  }

  /**
   * Imediately stop all scheduled and playing sounds
   */
  public stopAll() {
    for (const instruemnt of this.instruments.values()) {
      instruemnt.stopAll();
    }
  }

  /**
   * Mute an instrument.
   *
   * This will take into account any solo instruments.
   */
  public mute(key: string) {
    const current = this.instruments.get(key);
    if (current) {
      current.mute = true;
    }
    this.setSamplerMuteStates();
  }

  /**
   * Unmute an instrument.
   *
   * This will take into account any solo instruments.
   */
  public unmute(key: string) {
    const current = this.instruments.get(key);
    if (current) {
      current.mute = false;
    }
    this.setSamplerMuteStates();
  }

  /**
   * Solo an instrument.
   *
   * This will take into account any muted instruments.
   */
  public solo(key: string) {
    const current = this.instruments.get(key);
    if (current) {
      current.solo = true;
    }
    this.setSamplerMuteStates();
  }

  /**
   * Unsolo an instrument.
   *
   * This will take into account any muted instruments.
   */
  public unsolo(key: string) {
    const current = this.instruments.get(key);
    if (current) {
      current.solo = false;
    }
    this.setSamplerMuteStates();
  }

  /**
   * Set the volume of an instrument (0.0 - 1.0);
   */
  public volume(key: string, value: number) {
    const current = this.instruments.get(key);
    if (current) {
      current.volume(value);
    }
  }

  /**
   * Disconnect all the playback nodes so they can be garbage collected
   */
  public disconnect(key: string) {
    const current = this.instruments.get(key);
    if (current) {
      current.disconnectAll;
      this.instruments.delete(key);
    }
  }
}
