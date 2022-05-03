/* tslint:disable */
/* eslint-disable */
/**
* Get a full path to def from partial path
* @param {any} selection
* @returns {any}
*/
export function get_full_path_from_partial(selection: any): any;
/**
* Get a tree of instruments from a (possibly incomplete) path
* @param {any} selection
* @returns {any}
*/
export function def_tree(selection: any): any;
/**
*/
export function run(): void;
/**
*/
export enum NoteDuration {
  Whole,
  Half,
  Quarter,
  Eighth,
  Sixteenth,
  ThirtySecond,
  SixtyFourth,
}
/**
*/
export enum Articulation {
  None,
  Staccato,
  Staccatissimo,
  Tenuto,
  StaccatoTenuto,
}
/**
*/
export enum AutoCountStyle {
  Arabic,
  Roman,
}
/**
*/
export enum Expression {
  Natural,
  Pizzicato,
  Spiccato,
  Staccato,
  Tremolo,
  Mute,
}
/**
*/
export enum InstrumentType {
  Melodic,
  Percussive,
}
/**
*/
export enum PlayerType {
  Solo,
  Section,
}
/**
*/
export enum KeySignatureMode {
  Major,
  Minor,
}
/**
*/
export enum Accidental {
  DoubleSharp,
  Sharp,
  Natural,
  Flat,
  DoubleFlat,
}
/**
*/
export enum BarlineType {
  Double,
  EndRepeat,
  EndStartRepeat,
  Final,
  Normal,
  StartRepeat,
}
/**
*/
export enum ClefDrawType {
  Hidden,
  G,
  F,
  C,
  Percussion,
}
/**
*/
export enum TimeSignatureDrawType {
  Hidden,
  Normal,
  CommonTime,
  SplitCommonTime,
  Open,
}
/**
*/
export enum BracketingApproach {
  None,
  Orchestral,
  SmallEnsemble,
}
/**
*/
export enum BracketStyle {
  None,
  Wing,
  Line,
}
/**
*/
export enum LayoutType {
  Score,
  Part,
  Custom,
}
/**
*/
export class Engine {
  free(): void;
/**
* @param {string} flow_key
* @param {number} px_per_mm
* @param {Function} measure
* @returns {any}
*/
  render(flow_key: string, px_per_mm: number, measure: Function): any;
/**
* @param {number} player_type
* @returns {string}
*/
  create_player(player_type: number): string;
/**
* @param {string} player_key
*/
  remove_player(player_key: string): void;
/**
* @param {number} from
* @param {number} to
*/
  reorder_players(from: number, to: number): void;
/**
* @param {string} player_key
* @param {string} instrument_key
*/
  assign_instrument_to_player(player_key: string, instrument_key: string): void;
/**
* @param {string} player_key
* @param {string} instrument_key
*/
  unassign_instrument_from_player(player_key: string, instrument_key: string): void;
/**
* @param {string} player_key
* @param {number} from
* @param {number} to
*/
  reorder_player_instruments(player_key: string, from: number, to: number): void;
/**
* @param {string} player_key
* @returns {number}
*/
  get_player_type(player_key: string): number;
/**
* @param {string} player_key
* @returns {string}
*/
  get_player_name(player_key: string): string;
/**
* @param {string} player_key
* @returns {any}
*/
  get_player_instruments(player_key: string): any;
/**
* @param {string} flow_key
* @param {number} tick
* @param {number} mode
* @param {number} offset
*/
  create_key_signature(flow_key: string, tick: number, mode: number, offset: number): void;
/**
* Create a tone
* @param {string} track_key
* @param {number} tick
* @param {number} duration
* @param {number} pitch
* @param {number} velocity
* @param {number} articulation
* @returns {string}
*/
  create_tone(track_key: string, tick: number, duration: number, pitch: number, velocity: number, articulation: number): string;
/**
* update tone pitch
* @param {string} track_key
* @param {string} entry_key
* @param {number} pitch
*/
  set_tone_pitch(track_key: string, entry_key: string, pitch: number): void;
/**
* update tone duration
* @param {string} track_key
* @param {string} entry_key
* @param {number} duration
*/
  set_tone_duration(track_key: string, entry_key: string, duration: number): void;
/**
* move the tone
* @param {string} track_key
* @param {string} entry_key
* @param {number} new_tick
*/
  shift_tone(track_key: string, entry_key: string, new_tick: number): void;
/**
* Remove the tone
* @param {string} track_key
* @param {string} entry_key
*/
  remove_tone(track_key: string, entry_key: string): void;
/**
* Slice a tone
* @param {string} track_key
* @param {string} entry_key
* @param {number} slice_at
*/
  slice_tone(track_key: string, entry_key: string, slice_at: number): void;
/**
* @param {string} track_key
* @returns {any}
*/
  get_tones(track_key: string): any;
/**
* @param {string} flow_key
* @param {string} instrument_key
* @returns {any}
*/
  get_all_tones(flow_key: string, instrument_key: string): any;
/**
* @returns {string}
*/
  create_flow(): string;
/**
* @param {string} flow_key
*/
  remove_flow(flow_key: string): void;
/**
* @param {number} old_index
* @param {number} new_index
*/
  reorder_flow(old_index: number, new_index: number): void;
/**
* @param {string} flow_key
* @param {string} name
*/
  rename_flow(flow_key: string, name: string): void;
/**
* @param {string} flow_key
* @param {number} length
*/
  set_flow_length(flow_key: string, length: number): void;
/**
*
*     * Assign a player to a flow
*     
* @param {string} flow_key
* @param {string} player_key
*/
  assign_player_to_flow(flow_key: string, player_key: string): void;
/**
*
*     * Assign instrument to flow
*     
* @param {string} flow_key
* @param {string} instrument_key
*/
  assign_instrument_to_flow(flow_key: string, instrument_key: string): void;
/**
* @param {string} flow_key
* @param {string} player_key
*/
  unassign_player_from_flow(flow_key: string, player_key: string): void;
/**
* @param {string} flow_key
* @param {string} instrument_key
*/
  unassign_instrument_from_flow(flow_key: string, instrument_key: string): void;
/**
* @param {string} flow_key
* @returns {string}
*/
  get_flow_title(flow_key: string): string;
/**
* @param {string} flow_key
* @param {string} player_key
* @returns {boolean}
*/
  flow_contains_player(flow_key: string, player_key: string): boolean;
/**
* @param {string} flow_key
* @returns {any}
*/
  get_flow_ticks(flow_key: string): any;
/**
*/
  constructor();
/**
* @param {Function} cb
*/
  listen(cb: Function): void;
/**
* @param {string} flow_key
* @param {number} tick
* @param {number} beats
* @param {number} beat_type
* @param {number} draw_type
* @param {Uint8Array | undefined} groupings
*/
  create_time_signature(flow_key: string, tick: number, beats: number, beat_type: number, draw_type: number, groupings?: Uint8Array): void;
/**
* @param {number} layout_type
* @param {string} name
*/
  create_engrave(layout_type: number, name: string): void;
/**
* @param {string} key
* @returns {boolean}
*/
  get_systemic_barline_single_instrument_system(key: string): boolean;
/**
* @param {string} key
* @param {boolean} value
*/
  set_systemic_barline_single_instrument_system(key: string, value: boolean): void;
/**
* @param {string} key
* @returns {number}
*/
  get_bracketing_approach(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_bracketing_approach(key: string, value: number): void;
/**
* @param {string} key
* @returns {number}
*/
  get_bracket_style(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_bracket_style(key: string, value: number): void;
/**
* @param {string} key
* @returns {boolean}
*/
  get_bracket_single_staves(key: string): boolean;
/**
* @param {string} key
* @param {boolean} value
*/
  set_bracket_single_staves(key: string, value: boolean): void;
/**
* @param {string} key
* @returns {boolean}
*/
  get_sub_bracket(key: string): boolean;
/**
* @param {string} key
* @param {boolean} value
*/
  set_sub_bracket(key: string, value: boolean): void;
/**
* @param {string} key
* @returns {number}
*/
  get_base_note_space(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_base_note_space(key: string, value: number): void;
/**
* @param {string} key
* @returns {number}
*/
  get_minimum_note_space(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_minimum_note_space(key: string, value: number): void;
/**
* @param {string} key
* @returns {number}
*/
  get_minimum_tie_space(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_minimum_tie_space(key: string, value: number): void;
/**
* @param {string} key
* @returns {number}
*/
  get_note_space_ratio(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_note_space_ratio(key: string, value: number): void;
/**
* @param {string} key
* @returns {number}
*/
  get_space(key: string): number;
/**
* @param {string} key
* @param {number} value
*/
  set_space(key: string, value: number): void;
/**
* Create an instrument
* @param {string} id
* @returns {string}
*/
  create_instrument(id: string): string;
/**
* @param {string} instrument_key
*/
  remove_instrument(instrument_key: string): void;
/**
* @param {string} instrument_key
* @returns {string}
*/
  get_instrument_name(instrument_key: string): string;
/**
* @param {string} instrument_key
* @returns {string}
*/
  get_instrument_id(instrument_key: string): string;
/**
* @param {string} instrument_key
* @returns {number}
*/
  get_instrument_volume(instrument_key: string): number;
/**
* @param {string} instrument_key
* @param {number} value
*/
  set_instrument_volume(instrument_key: string, value: number): void;
/**
* @param {string} instrument_key
* @returns {boolean}
*/
  get_instrument_solo(instrument_key: string): boolean;
/**
* @param {string} instrument_key
*/
  toggle_instrument_solo(instrument_key: string): void;
/**
* @param {string} instrument_key
* @returns {boolean}
*/
  get_instrument_mute(instrument_key: string): boolean;
/**
* @param {string} instrument_key
*/
  toggle_instrument_mute(instrument_key: string): void;
/**
* @param {string} instrument_key
* @returns {any}
*/
  get_instrument_staves(instrument_key: string): any;
/**
* @param {string} flow_key
* @param {string} instrument_key
* @returns {any}
*/
  get_instrument_tracks(flow_key: string, instrument_key: string): any;
/**
*/
  calculate_counts(): void;
/**
* @returns {string}
*/
  application_version: string;
/**
* @returns {string}
*/
  arranger: string;
/**
* @returns {number}
*/
  auto_count_style_section: number;
/**
* @returns {number}
*/
  auto_count_style_solo: number;
/**
* @returns {string}
*/
  composer: string;
/**
* @returns {string}
*/
  copyright: string;
/**
* @returns {number}
*/
  created: number;
/**
* @returns {any}
*/
  readonly engraves: any;
/**
* @returns {any}
*/
  readonly flows: any;
/**
* @returns {string}
*/
  lyricist: string;
/**
* @returns {any}
*/
  readonly players: any;
/**
* @returns {string}
*/
  readonly state: string;
/**
* @returns {string}
*/
  subtitle: string;
/**
* @returns {string}
*/
  title: string;
}
/**
*/
export class Pitch {
  free(): void;
/**
*/
  accidental: number;
/**
*/
  int: number;
}
/**
*/
export class Velocity {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly engine_render: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly __wbg_velocity_free: (a: number) => void;
  readonly get_full_path_from_partial: (a: number) => number;
  readonly def_tree: (a: number) => number;
  readonly engine_application_version: (a: number, b: number) => void;
  readonly engine_set_application_version: (a: number, b: number, c: number) => void;
  readonly engine_title: (a: number, b: number) => void;
  readonly engine_set_title: (a: number, b: number, c: number) => void;
  readonly engine_subtitle: (a: number, b: number) => void;
  readonly engine_set_subtitle: (a: number, b: number, c: number) => void;
  readonly engine_composer: (a: number, b: number) => void;
  readonly engine_set_composer: (a: number, b: number, c: number) => void;
  readonly engine_arranger: (a: number, b: number) => void;
  readonly engine_set_arranger: (a: number, b: number, c: number) => void;
  readonly engine_lyricist: (a: number, b: number) => void;
  readonly engine_set_lyricist: (a: number, b: number, c: number) => void;
  readonly engine_copyright: (a: number, b: number) => void;
  readonly engine_set_copyright: (a: number, b: number, c: number) => void;
  readonly engine_created: (a: number) => number;
  readonly engine_set_created: (a: number, b: number) => void;
  readonly engine_auto_count_style_solo: (a: number) => number;
  readonly engine_set_auto_count_style_solo: (a: number, b: number) => void;
  readonly engine_auto_count_style_section: (a: number) => number;
  readonly engine_set_auto_count_style_section: (a: number, b: number) => void;
  readonly engine_create_player: (a: number, b: number, c: number) => void;
  readonly engine_remove_player: (a: number, b: number, c: number) => void;
  readonly engine_reorder_players: (a: number, b: number, c: number) => void;
  readonly engine_assign_instrument_to_player: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_unassign_instrument_from_player: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_reorder_player_instruments: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_players: (a: number) => number;
  readonly engine_get_player_type: (a: number, b: number, c: number) => number;
  readonly engine_get_player_name: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_player_instruments: (a: number, b: number, c: number) => number;
  readonly engine_create_key_signature: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_pitch_free: (a: number) => void;
  readonly __wbg_get_pitch_int: (a: number) => number;
  readonly __wbg_set_pitch_int: (a: number, b: number) => void;
  readonly __wbg_get_pitch_accidental: (a: number) => number;
  readonly __wbg_set_pitch_accidental: (a: number, b: number) => void;
  readonly engine_create_tone: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly engine_set_tone_pitch: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly engine_set_tone_duration: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly engine_shift_tone: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly engine_remove_tone: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_slice_tone: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly engine_get_tones: (a: number, b: number, c: number) => number;
  readonly engine_get_all_tones: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly engine_create_flow: (a: number, b: number) => void;
  readonly engine_remove_flow: (a: number, b: number, c: number) => void;
  readonly engine_reorder_flow: (a: number, b: number, c: number) => void;
  readonly engine_rename_flow: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_set_flow_length: (a: number, b: number, c: number, d: number) => void;
  readonly engine_assign_player_to_flow: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_assign_instrument_to_flow: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_unassign_player_from_flow: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_unassign_instrument_from_flow: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly engine_flows: (a: number) => number;
  readonly engine_get_flow_title: (a: number, b: number, c: number, d: number) => void;
  readonly engine_flow_contains_player: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly engine_get_flow_ticks: (a: number, b: number, c: number) => number;
  readonly __wbg_engine_free: (a: number) => void;
  readonly engine_new: () => number;
  readonly engine_listen: (a: number, b: number) => void;
  readonly engine_state: (a: number, b: number) => void;
  readonly run: () => void;
  readonly engine_create_time_signature: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly engine_create_engrave: (a: number, b: number, c: number, d: number) => void;
  readonly engine_engraves: (a: number) => number;
  readonly engine_get_systemic_barline_single_instrument_system: (a: number, b: number, c: number) => number;
  readonly engine_set_systemic_barline_single_instrument_system: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_bracketing_approach: (a: number, b: number, c: number) => number;
  readonly engine_set_bracketing_approach: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_bracket_style: (a: number, b: number, c: number) => number;
  readonly engine_set_bracket_style: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_bracket_single_staves: (a: number, b: number, c: number) => number;
  readonly engine_set_bracket_single_staves: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_sub_bracket: (a: number, b: number, c: number) => number;
  readonly engine_set_sub_bracket: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_base_note_space: (a: number, b: number, c: number) => number;
  readonly engine_set_base_note_space: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_minimum_note_space: (a: number, b: number, c: number) => number;
  readonly engine_set_minimum_note_space: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_minimum_tie_space: (a: number, b: number, c: number) => number;
  readonly engine_set_minimum_tie_space: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_note_space_ratio: (a: number, b: number, c: number) => number;
  readonly engine_set_note_space_ratio: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_space: (a: number, b: number, c: number) => number;
  readonly engine_set_space: (a: number, b: number, c: number, d: number) => void;
  readonly engine_create_instrument: (a: number, b: number, c: number, d: number) => void;
  readonly engine_remove_instrument: (a: number, b: number, c: number) => void;
  readonly engine_get_instrument_name: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_instrument_id: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_instrument_volume: (a: number, b: number, c: number) => number;
  readonly engine_set_instrument_volume: (a: number, b: number, c: number, d: number) => void;
  readonly engine_get_instrument_solo: (a: number, b: number, c: number) => number;
  readonly engine_toggle_instrument_solo: (a: number, b: number, c: number) => void;
  readonly engine_get_instrument_mute: (a: number, b: number, c: number) => number;
  readonly engine_toggle_instrument_mute: (a: number, b: number, c: number) => void;
  readonly engine_get_instrument_staves: (a: number, b: number, c: number) => number;
  readonly engine_get_instrument_tracks: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly engine_calculate_counts: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
