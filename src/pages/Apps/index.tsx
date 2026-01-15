import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { LuPlus } from "react-icons/lu";

import AppItem from "~/components/apps/AppItem";
import Expander from "~/components/common/Expander";
import { Item } from "~/components/common/Item";
import Page from "~/components/common/Page";
import { useStickyState } from "~/hooks/useStickyState";
import { type App, PageType } from "~/types";

interface AppsProps {
	page: PageType;
	search: string;
}

export default function Apps({ page, search }: AppsProps) {
	const [apps, setApps] = useStickyState<App[]>("apps", []);

	const appWindow = getCurrentWindow();

	const filteredApps = apps.filter((app) =>
		app.label.toLowerCase().includes(search.toLowerCase()),
	);

	const addApp = async () => {
		const paths = await open({
			multiple: true,
			filters: [
				{
					name: "",
					extensions: ["lnk", "exe"],
				},
			],
		});

		if (!paths) return;

		setApps((prev) => {
			const newApps = paths
				.filter((path) => !prev.some((app) => app.path === path))
				.map((path) => {
					const file = path.split(/(\\|\/)/g).pop()!;
					const label = file.replace(/\.[^/.]+$/, "");
					return { label, path };
				});

			return [...prev, ...newApps];
		});

		appWindow.show();
		appWindow.setFocus();
	};

	return (
		<Page
			target={PageType.APPS}
			current={page}
			className="gap-3"
		>
			<Item
				icon={<LuPlus />}
				title="Add a new app"
				description=""
				onClick={addApp}
			/>

			{filteredApps.length !== 0 && (
				<Expander label="Apps">
					{filteredApps.map((app) => (
						<AppItem
							key={app.path}
							app={app}
							apps={apps}
							setApps={setApps}
						/>
					))}
				</Expander>
			)}
		</Page>
	);
}
