import { registerControllers, html } from "/js/vendor/binder/binder.js";
import { DynamicFrame } from "/js/vendor/binder/core/dynamic_frame.js";
import { Config } from "/js/fn.js";
import { MediaList } from "./controllers/media_list.js";

registerControllers(DynamicFrame, MediaList);

window.addEventListener("DOMContentLoaded", async () => {
	const config = await Config.get();
	const main = document.querySelector("main");

	if (!config.source) {
		main.innerHTML = html`<dynamic-frame :url="/frames/setup.html" :execute-scripts></dynamic-frame>`;
	} else {
		main.innerHTML = html`<dynamic-frame :url="/frames/media.html" :execute-scripts></dynamic-frame>`;
	}

	// Reset application (for debugging)
	const reset = document.querySelector("#reset");
	reset.addEventListener("click", async () => {
		await Config.delete();

		window.location.reload();
	});
});
