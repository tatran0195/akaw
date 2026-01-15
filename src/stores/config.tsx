import { create } from "zustand";

import type { AppConfig, AppearanceSettings } from "~/types";

export const CONFIG_VERSION = 1;

export function createBaseConfig(): AppConfig {
	return {
		appearance: {
			locale: "en",
		},
		apps: [],
		commands: {},
	};
}

export type ConfigStore = AppConfig & {
	applyPreference: <T>(updater: (state: ConfigStore, value: T) => void, value: T) => void;
	updateAppearanceSettings: (settings: Partial<AppearanceSettings>) => void;
	resetConfig: () => void;
};

export const useConfigStore = create<ConfigStore>()((set) => ({
	...createBaseConfig(),

	applyPreference: (updater, value) =>
		set((state) => {
			updater(state, value);
			return state;
		}),

	updateAppearanceSettings: (settings) =>
		set((state) => ({
			...state,
			appearance: {
				...state.appearance,
				...settings,
			},
		})),

	resetConfig: () => {
		return set(() => ({
			...createBaseConfig(),
		}));
	},
}));
