const { invoke } = window.__TAURI__.tauri;

import { registerControllers } from "/js/vendor/binder/register.js";
import { TauriFrame } from "./controllers/tauri-frame.js";

registerControllers(TauriFrame);

async function getConfig() {
	return await invoke("get_config", {});
}

window.addEventListener("DOMContentLoaded", async () => {
	const config = await getConfig();
	if (!config.initialised) {
		console.log("No config found, redirecting to wizard...");
		window.location = "/pages/wizard.html";
	}
});
