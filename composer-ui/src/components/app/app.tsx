import { FC, useEffect } from "react";
import { engine } from "../..";

interface Props {}

export const App: FC<Props> = () => {
  return (
    <>
      <input onChange={(e) => (engine.title = e.target.value)} />
      <p>title: {engine.title}</p>
      <p>modified: {engine.modified}</p>
      <p>modified: {engine.created}</p>
    </>
  );
};
