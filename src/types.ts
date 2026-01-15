export enum PageType {
	HOME,
	APPS,
	AWS,
	COMMANDS,
}

export interface App {
	label: string;
	path: string;
}

export interface Command {
	label: string;
	command: string;
	admin: boolean;
}

export interface CommandList {
	[category: string]: Command[];
}

export interface AppearanceSettings {
	locale: string;
}

export interface AppConfig {
	appearance: AppearanceSettings;
	apps: App[];
	commands: CommandList;
}
