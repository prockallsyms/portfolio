// Boots the wasm package, then starts the Yew app (see src/lib.rs).
import init, { run } from "./pkg/portfolio.js";

init.then(() => run());
