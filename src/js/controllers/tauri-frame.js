const { readTextFile } = window.__TAURI__.fs;
const { resolveResource } = window.__TAURI__.path;
import { DynamicFrame } from "/js/vendor/binder/core/dynamic_frame.js";

/**
 * Wrapper around `DynamicFrame` that loads resources from the disk instead of via HTTP
 */
class TauriFrame extends DynamicFrame {
	async loadContent(e) {
		const url = this.args.url;

		const resource = await resolveResource(url);
		const content = await readTextFile(resource);

		this.updateContent(content);
		this.bind();

		return true;
	}
}

export { TauriFrame };
