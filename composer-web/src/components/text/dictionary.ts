import { engine } from "../../data";

export const variables: Map<string, () => string> = new Map([
  // generated
  ["${year}", () => new Date().getFullYear().toString()],
  ["${month}", () => new Date().getMonth().toString().padStart(2, "0")],
  ["${date}", () => new Date().getDate().toString().padStart(2, "0")],
  // user-defined
  ["${project-title}", () => engine.title],
  ["${project-subtitle}", () => engine.subtitle],
  ["${project-composer}", () => engine.composer],
  ["${project-arranger}", () => engine.arranger],
  ["${project-lyricist}", () => engine.lyricist],
  ["${project-copyright}", () => engine.copyright],
]);

export const symbols: Map<string, { content: string; sym: boolean }> = new Map([
  // non-musical
  ["${copy}", { content: "\u{00A9}", sym: false }],

  // musical
  ["${whole}", { content: "\u{E1D2}", sym: true }],
  ["${half}", { content: "\u{E1D3}", sym: true }],
  ["${quarter}", { content: "\u{E1D5}", sym: true }],
  ["${eighth}", { content: "\u{E1D7}", sym: true }],
  ["${sixteenth}", { content: "\u{E1D9}", sym: true }],
  ["${thirtysecond}", { content: "\u{E1DB}", sym: true }],

  ["${dot}", { content: "\u{E1E7}", sym: true }],

  ["${sharp}", { content: "\u{E262}", sym: true }],
  ["${natural}", { content: "\u{E261}", sym: true }],
  ["${flat}", { content: "\u{E260}", sym: true }],

  ["${time-0}", { content: "\u{E080}", sym: true }],
  ["${time-1}", { content: "\u{E081}", sym: true }],
  ["${time-2}", { content: "\u{E082}", sym: true }],
  ["${time-3}", { content: "\u{E083}", sym: true }],
  ["${time-4}", { content: "\u{E084}", sym: true }],
  ["${time-5}", { content: "\u{E085}", sym: true }],
  ["${time-6}", { content: "\u{E086}", sym: true }],
  ["${time-7}", { content: "\u{E087}", sym: true }],
  ["${time-8}", { content: "\u{E088}", sym: true }],
  ["${time-9}", { content: "\u{E089}", sym: true }],
  ["${time-c}", { content: "\u{E08A}", sym: true }],
  ["${time-cutc}", { content: "\u{E08B}", sym: true }],
]);
