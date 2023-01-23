import { css, html, Controller } from "/js/vendor/binder/binder.js";
import { Config, IO } from "/js/fn.js";

/**
 * Main container for the list of media items
 */
class MediaList extends Controller {
	async init() {
		this.config = await Config.get();
		this.files = await this.getFiles();

		const files = this.files.map(
			file => html`<div class="row">
				<div class="col mb-md">
					<span class="no-break">${file.name}</span>
					<span class="fs-sm font-mono no-break">${file.path}</span>
				</div>
			</div>`
		);
		this.innerHTML = html` <div id="main">${files.join("")}</div>`;
	}

	async getFiles() {
		return await IO.listFiles(this.config.source.path);
	}
}

export { MediaList };
