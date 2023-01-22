import { html, Controller } from "/js/vendor/binder/binder.js";
import { Config, IO } from "/js/fn.js";

/**
 * Main container for the list of media items
 */
class MediaList extends Controller {
	async init() {
		this.config = await Config.get();
		this.files = await this.getFiles();

		const files = this.files.map(file => html`<p>${file.name}</p>`);
		this.innerHTML = html`<div>${files.join("")}</div>`;
	}

	async getFiles() {
		return await IO.listFiles(this.config.source.path);
	}
}

export { MediaList };
