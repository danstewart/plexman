import { registerControllers, html } from "/js/vendor/binder/binder.js";
import { DynamicFrame } from "/js/vendor/binder/core/dynamic_frame.js";
import { getConfig, writeConfig } from "./fn.js";

registerControllers(DynamicFrame);

window.addEventListener("DOMContentLoaded", async () => {
	const config = await getConfig();
	const main = document.querySelector("main");

	if (!config.source) {
		const welcome = document.createElement("div");
		welcome.innerHTML = html`
			<p>Welcome to PlexMan!</p>
			<p>To get started add a new source using the button above.</p>
		`;

		main.appendChild(welcome);
	}

	const addSource = document.querySelector("#addSource");
	addSource.addEventListener("click", async () => {
		await writeConfig({
			source: {
				path: "/xyz",
			},
			targets: [],
		});

		window.location.reload();
	});
});
