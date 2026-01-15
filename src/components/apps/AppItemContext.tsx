import { ContextMenu, ContextMenuItem } from "~/components/common/ContextMenu";

interface AppItemContextProps {
	x: number;
	y: number;
	closeContextMenu: () => void;
	openApp: () => Promise<void>;
	openInExplorer: () => Promise<void>;
	removeApp: () => Promise<void>;
}

export function AppItemContext({
	x,
	y,
	closeContextMenu,
	openApp,
	openInExplorer,
	removeApp,
}: AppItemContextProps) {
	return (
		<ContextMenu
			x={x}
			y={y}
			closeContextMenu={closeContextMenu}
		>
			<ContextMenuItem
				label="Open"
				onClick={openApp}
			/>
			<ContextMenuItem
				label="Open in explorer"
				onClick={openInExplorer}
			/>
			<ContextMenuItem
				label="Remove app"
				onClick={removeApp}
			/>
		</ContextMenu>
	);
}
