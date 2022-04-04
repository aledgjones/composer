import { createRoot } from "react-dom/client";
import { Root } from "./states/root/root";
import { engine } from "./engine";

import "./fonts/fonts.css";
import "./ui";

const container = document.querySelector("#app");
const root = createRoot(container);

engine.listen(() => {
  root.render(<Root />);
});
