import { createRoot } from "react-dom/client";
import { Root } from "./states/root/root";
import { engine } from "./data";

import "./fonts/fonts.css";
import "./ui";

const container = document.querySelector("#app");
if (container) {
  const root = createRoot(container);
  engine.listen(() => {
    root.render(<Root />);
  });
}
