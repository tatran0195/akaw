import type React from "react";
import { useState } from "react";
import { LuPackage } from "react-icons/lu";

import { Item } from "~/components/common/Item";
import { exec } from "~/services/api/tauri";
import type { App } from "~/types";

import { AppItemContext } from "./AppItemContext";

const initialContextMenu = {
	x: 0,
	y: 0,
	visible: false,
};

interface AppItemProps {
	app: App;
	apps: App[];
	setApps: React.Dispatch<React.SetStateAction<App[]>>;
}

export default function AppItem({ app, apps, setApps }: AppItemProps) {
	const [context, setContext] = useState(initialContextMenu);

	const closeContextMenu = () => setContext(initialContextMenu);

	const handleContextMenu = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
		e.preventDefault();

		const x = e.clientX;
		const y = e.clientY;

		setContext({ x, y, visible: true });
	};

	const openApp = async () => exec(app.path, false);

	const openInExplorer = async () => {
		const path = app.path.substring(0, app.path.lastIndexOf("\\"));

		exec(`explorer "${path}"`, false);
	};

	const removeApp = async () => {
		const newApps = apps.filter((a) => a.path !== app.path);
		setApps(newApps);
		localStorage.setItem("apps", JSON.stringify(newApps));
	};

	return (
		<>
			<Item
				icon={<LuPackage />}
				title={app.label}
				description={app.path}
				onClick={openApp}
				onContextMenu={handleContextMenu}
			/>

			{context.visible && (
				<AppItemContext
					x={context.x}
					y={context.y}
					closeContextMenu={closeContextMenu}
					openApp={openApp}
					openInExplorer={openInExplorer}
					removeApp={removeApp}
				/>
			)}
		</>
	);
}
