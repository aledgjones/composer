import { createRoot } from "react-dom/client";
import { Root } from "./states/root/root";
import { engine } from "./data";

import "./fonts/fonts.css";
import "./ui";
import { player } from "./sampler";

const sampler = player.createSampler("one");
sampler.load(0, "/patches/piano/natural.json", () => true);
(window as any).sampler = sampler;

const container = document.querySelector("#app");
if (container) {
  const root = createRoot(container);
  engine.listen(() => {
    root.render(<Root />);
  });
}
