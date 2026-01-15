import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { useEffect, useState } from "react";

export function HomeUpdatePanel() {
	const [latestUpdate, setLatestUpdate] = useState<string>("");
	const [updateAvailable, setUpdateAvailable] = useState<boolean>(false);

	useEffect(() => {
		const fetchUpdate = async () => {
			try {
				const update = (await invoke("latest_update")) as string;
				const currentVersion = await getVersion();
				const available = currentVersion !== update;

				setLatestUpdate(update);
				setUpdateAvailable(available);
			} catch (error) {
				console.error(error);
			}
		};

		fetchUpdate();
	}, []);

	return (
		<>
			{updateAvailable && (
				<div className="w-full flex flex-col items-center justify-center gap-2 p-4 text-md bg-background-secondary border border-background-tertiary rounded-md">
					<p className="text-foreground-secondary">
						New version of akaw is available ({latestUpdate})
					</p>
					<a
						onClick={() => open("https://github.com/Bamboooz/akaw/releases/latest")}
						className="text-accent hover:underline cursor-pointer"
					>
						Check it out!
					</a>
				</div>
			)}
		</>
	);
}
