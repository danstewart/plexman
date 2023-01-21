import { registerControllers } from "/js/vendor/binder/register.js";
import { DynamicFrame } from "/js/vendor/binder/core/dynamic_frame.js";
import { getConfig } from "./fn.js";

registerControllers(DynamicFrame);

window.addEventListener("DOMContentLoaded", async () => {
	const config = await getConfig();
});
