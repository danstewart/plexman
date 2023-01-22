const invoke = async (method, args) => {
	const { invoke } = window.__TAURI__.tauri;

	try {
		return await invoke(method, args);
	} catch (err) {
		console.error("Error invoking method: " + method);
		console.error(err);
	}
};

// Config
const Config = {
	get: async () => invoke("get_config", {}),
	write: async config => invoke("write_config", { config }),
	delete: async () => invoke("delete_config", {}),
};

// IO
const IO = {
	listFiles: async path => invoke("list_files", { path }),
};

export { Config, IO };
