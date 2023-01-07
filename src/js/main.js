const { invoke } = window.__TAURI__.tauri;
const { readTextFile } = window.__TAURI__.fs;
const { resolveResource } = window.__TAURI__.path;

import { registerControllers } from "/js/vendor/binder/register.js";
import { TauriFrame } from "./controllers/tauri-frame.js";

registerControllers(TauriFrame);

async function listFiles(dir) {
	return await invoke("list_files", { dir: dir });
}
