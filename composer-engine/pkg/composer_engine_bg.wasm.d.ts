/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_engine_free(a: number): void;
export function engine_new(): number;
export function engine_listen(a: number, b: number): void;
export function engine_state(a: number, b: number): void;
export function run(): void;
export function engine_create_key_signature(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function engine_create_instrument(a: number, b: number, c: number, d: number): void;
export function engine_remove_instrument(a: number, b: number, c: number): void;
export function engine_get_instrument_name(a: number, b: number, c: number, d: number): void;
export function engine_get_instrument_id(a: number, b: number, c: number, d: number): void;
export function engine_get_instrument_volume(a: number, b: number, c: number): number;
export function engine_set_instrument_volume(a: number, b: number, c: number, d: number): void;
export function engine_get_instrument_solo(a: number, b: number, c: number): number;
export function engine_toggle_instrument_solo(a: number, b: number, c: number): void;
export function engine_get_instrument_mute(a: number, b: number, c: number): number;
export function engine_toggle_instrument_mute(a: number, b: number, c: number): void;
export function engine_get_instrument_staves(a: number, b: number, c: number): number;
export function engine_get_instrument_tracks(a: number, b: number, c: number, d: number, e: number): number;
export function engine_calculate_counts(a: number): void;
export function engine_create_tone(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): void;
export function engine_set_tone_pitch(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function engine_set_tone_duration(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function engine_shift_tone(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function engine_remove_tone(a: number, b: number, c: number, d: number, e: number): void;
export function engine_slice_tone(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function engine_get_tones(a: number, b: number, c: number): number;
export function engine_get_all_tones(a: number, b: number, c: number, d: number, e: number): number;
export function engine_create_engrave(a: number, b: number, c: number, d: number): void;
export function engine_engraves(a: number): number;
export function engine_get_systemic_barline_single_instrument_system(a: number, b: number, c: number): number;
export function engine_set_systemic_barline_single_instrument_system(a: number, b: number, c: number, d: number): void;
export function engine_get_bracketing_approach(a: number, b: number, c: number): number;
export function engine_set_bracketing_approach(a: number, b: number, c: number, d: number): void;
export function engine_get_bracket_style(a: number, b: number, c: number): number;
export function engine_set_bracket_style(a: number, b: number, c: number, d: number): void;
export function engine_get_bracket_single_staves(a: number, b: number, c: number): number;
export function engine_set_bracket_single_staves(a: number, b: number, c: number, d: number): void;
export function engine_get_sub_bracket(a: number, b: number, c: number): number;
export function engine_set_sub_bracket(a: number, b: number, c: number, d: number): void;
export function engine_get_base_note_space(a: number, b: number, c: number): number;
export function engine_set_base_note_space(a: number, b: number, c: number, d: number): void;
export function engine_get_minimum_note_space(a: number, b: number, c: number): number;
export function engine_set_minimum_note_space(a: number, b: number, c: number, d: number): void;
export function engine_get_minimum_tie_space(a: number, b: number, c: number): number;
export function engine_set_minimum_tie_space(a: number, b: number, c: number, d: number): void;
export function engine_get_note_space_ratio(a: number, b: number, c: number): number;
export function engine_set_note_space_ratio(a: number, b: number, c: number, d: number): void;
export function engine_get_space(a: number, b: number, c: number): number;
export function engine_set_space(a: number, b: number, c: number, d: number): void;
export function engine_application_version(a: number, b: number): void;
export function engine_set_application_version(a: number, b: number, c: number): void;
export function engine_title(a: number, b: number): void;
export function engine_set_title(a: number, b: number, c: number): void;
export function engine_subtitle(a: number, b: number): void;
export function engine_set_subtitle(a: number, b: number, c: number): void;
export function engine_composer(a: number, b: number): void;
export function engine_set_composer(a: number, b: number, c: number): void;
export function engine_arranger(a: number, b: number): void;
export function engine_set_arranger(a: number, b: number, c: number): void;
export function engine_lyricist(a: number, b: number): void;
export function engine_set_lyricist(a: number, b: number, c: number): void;
export function engine_copyright(a: number, b: number): void;
export function engine_set_copyright(a: number, b: number, c: number): void;
export function engine_created(a: number): number;
export function engine_set_created(a: number, b: number): void;
export function __wbg_pitch_free(a: number): void;
export function __wbg_get_pitch_int(a: number): number;
export function __wbg_set_pitch_int(a: number, b: number): void;
export function __wbg_get_pitch_accidental(a: number): number;
export function __wbg_set_pitch_accidental(a: number, b: number): void;
export function engine_create_time_signature(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number): void;
export function engine_render(a: number, b: number, c: number, d: number, e: number): number;
export function __wbg_velocity_free(a: number): void;
export function get_full_path_from_partial(a: number): number;
export function def_tree(a: number): number;
export function engine_create_player(a: number, b: number, c: number): void;
export function engine_remove_player(a: number, b: number, c: number): void;
export function engine_reorder_players(a: number, b: number, c: number): void;
export function engine_assign_instrument_to_player(a: number, b: number, c: number, d: number, e: number): void;
export function engine_unassign_instrument_from_player(a: number, b: number, c: number, d: number, e: number): void;
export function engine_reorder_player_instruments(a: number, b: number, c: number, d: number, e: number): void;
export function engine_players(a: number): number;
export function engine_get_player_type(a: number, b: number, c: number): number;
export function engine_get_player_name(a: number, b: number, c: number, d: number): void;
export function engine_get_player_instruments(a: number, b: number, c: number): number;
export function engine_auto_count_style_solo(a: number): number;
export function engine_set_auto_count_style_solo(a: number, b: number): void;
export function engine_auto_count_style_section(a: number): number;
export function engine_set_auto_count_style_section(a: number, b: number): void;
export function engine_create_flow(a: number, b: number): void;
export function engine_remove_flow(a: number, b: number, c: number): void;
export function engine_reorder_flow(a: number, b: number, c: number): void;
export function engine_rename_flow(a: number, b: number, c: number, d: number, e: number): void;
export function engine_set_flow_length(a: number, b: number, c: number, d: number): void;
export function engine_assign_player_to_flow(a: number, b: number, c: number, d: number, e: number): void;
export function engine_assign_instrument_to_flow(a: number, b: number, c: number, d: number, e: number): void;
export function engine_unassign_player_from_flow(a: number, b: number, c: number, d: number, e: number): void;
export function engine_unassign_instrument_from_flow(a: number, b: number, c: number, d: number, e: number): void;
export function engine_flows(a: number): number;
export function engine_get_flow_title(a: number, b: number, c: number, d: number): void;
export function engine_flow_contains_player(a: number, b: number, c: number, d: number, e: number): number;
export function engine_get_flow_ticks(a: number, b: number, c: number): number;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_start(): void;
