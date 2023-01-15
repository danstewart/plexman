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
const getConfig = async () => invoke("get_config", {});
const writeConfig = async config => invoke("write_config", { config });
const deleteConfig = async () => invoke("delete_config", {});

export { getConfig, writeConfig, deleteConfig };
