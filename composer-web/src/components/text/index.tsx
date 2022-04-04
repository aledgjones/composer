import { FC, Fragment } from "react";

import { symbols, variables } from "./dictionary";

import "./styles.css";

function injectVar(content: string) {
  const varRegex = /(\${[^\s@]*})/g;
  return content.replace(varRegex, (token) => {
    if (variables.has(token)) {
      return variables.get(token)();
    } else {
      return token;
    }
  });
}

function injectSym(content: string) {
  const symRegex = /(\${[^\s}]*})/g;
  const split = content.split(symRegex).filter((entry) => entry); // filter any empties
  return split.map((entry) => {
    if (symbols.has(entry)) {
      return symbols.get(entry);
    } else {
      return { content: entry, sym: false };
    }
  });
}

interface Props {
  content: string;
}

/**
 * Converts a string of text with tokens in a formed string eg.
 * This uses the music font for music symbols.
 *
 * "Carinet in B${flat}" -> "Clarinet in B♭"
 */
export const Text: FC<Props> = ({ content }) => {
  const replaced = injectVar(content);
  const tokens = injectSym(replaced);

  return (
    <>
      {tokens.map((token, i) => {
        if (token.sym) {
          return (
            <span key={i} className="text--music-symbol">
              {token.content}
            </span>
          );
        } else {
          return <Fragment key={i}>{token.content}</Fragment>;
        }
      })}
    </>
  );
};
