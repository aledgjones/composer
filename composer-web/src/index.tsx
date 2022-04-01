import { createRoot } from "react-dom/client";
import { Engine } from "composer-engine";
import { App } from "./components/app/app";

export const engine = new Engine();

const container = document.querySelector("#app");
const root = createRoot(container);

engine.listen(() => {
  root.render(<App />);
});
