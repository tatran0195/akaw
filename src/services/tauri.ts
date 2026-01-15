import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

type COMMAND =
	// commands
	| "execute_command"
	| "fetch_commands"
	// aws
	| "list_aws_profiles"
	| "show_aws_config"
	| "setup_mfa_device"
	| "connect"
	| "remove_aws_profile"
	| "generate_totp_code"
	| "remove_mfa_device"
	| "get_profile_names"
	| "check_mfa_status"
	| "init_aws_configs"
	| "show_aws_config"
	// config
	| "load_config"
	| "save_config";

export async function invoke<T>(cmd: COMMAND, args?: any, hide: boolean = true): Promise<T> {
	if (hide) {
		const appWindow = getCurrentWindow();
		await appWindow.hide();
	}

	return tauriInvoke<T>(cmd, args);
}

export async function exec(command: string, admin: boolean = false) {
	await invoke("execute_command", { command, admin });
}
