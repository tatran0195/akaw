import { assign, debounce, isEmpty } from "radash";
import type { StoreApi, UseBoundStore } from "zustand";

import { invoke } from "~/services/tauri";
import { createBaseConfig, useConfigStore } from "~/stores/config";
import type { AppConfig } from "~/types";

export type Category = keyof Pick<AppConfig, "appearance">;
export type Settings<T extends Category> = AppConfig[T];
export type StoreType<T> = T extends UseBoundStore<StoreApi<infer I>> ? I : never;
export type ConfigFields = (keyof AppConfig)[];

/**
 * Get a single setting from the config store
 *
 * @param category The category
 * @param key The setting key
 * @returns Setting value
 */
export function getSetting<C extends Category, K extends keyof Settings<C>>(category: C, key: K) {
	return useConfigStore.getState()[category][key];
}

let skipConfigSave = false;

/**
 * Start the config synchronization process
 */
export async function startConfigSync() {
	const loadedConfig = await loadConfig();
	const preAssignedConfig = isEmpty(loadedConfig) ? createBaseConfig() : loadedConfig;
	const config = assign<AppConfig>(useConfigStore.getState(), preAssignedConfig);

	const scheduleSave = debounce(
		{
			delay: 250,
		},
		(state: AppConfig) => {
			saveConfig(state);
		},
	);

	useConfigStore.setState(config);

	useConfigStore.subscribe(async (updated) => {
		if (!skipConfigSave) {
			scheduleSave(updated);
		}
	});
}

/**
 * Overwrite the config store without triggering a save
 */
export function overwriteConfig(config: AppConfig) {
	if (!skipConfigSave) {
		try {
			skipConfigSave = true;
			useConfigStore.setState(config);
		} finally {
			skipConfigSave = false;
		}
	}
}

async function loadConfig() {
	return await invoke<AppConfig>("load_config");
}

async function saveConfig(config: AppConfig) {
	await invoke("save_config", config);
}
