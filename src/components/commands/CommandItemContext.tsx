import { ContextMenu, ContextMenuItem } from "~/components/common/ContextMenu";

interface CommandItemContextProps {
	x: number;
	y: number;
	closeContextMenu: () => void;
	execute: (admin: boolean) => Promise<void>;
}

export default function CommandItemContext({
	x,
	y,
	closeContextMenu,
	execute,
}: CommandItemContextProps) {
	return (
		<ContextMenu
			x={x}
			y={y}
			closeContextMenu={closeContextMenu}
		>
			<ContextMenuItem
				label="Execute"
				onClick={() => execute(false)}
			/>
			<ContextMenuItem
				label="Execute as admin"
				onClick={() => execute(true)}
			/>
		</ContextMenu>
	);
}
