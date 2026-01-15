import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export async function invoke<T>(cmd: string, args?: any, hide: boolean = true): Promise<T> {
	if (hide) {
		const appWindow = getCurrentWindow();
		await appWindow.hide();
	}

	return tauriInvoke<T>(cmd, args);
}

export async function exec(command: string, admin: boolean = false) {
	await invoke("execute_command", { command, admin });
}
