import { registerControllers } from "/js/vendor/binder/register.js";
import { TauriFrame } from "./controllers/tauri-frame.js";
import { getConfig } from "./fn.js";

registerControllers(TauriFrame);

window.addEventListener("DOMContentLoaded", async () => {
	const config = await getConfig();
	if (!config.initialised) {
		console.log("No config found, redirecting to wizard...");
		window.location = "/pages/wizard.html";
	}
});
