interface ContextMenuItemProps {
	label: string;
	onClick: () => void;
}

export function ContextMenuItem({ label, onClick }: ContextMenuItemProps) {
	return (
		<button
			onClick={onClick}
			className="w-full h-8 flex items-center px-4 rounded-md hover:bg-background-secondary"
		>
			<p className="text-foreground-secondary text-md truncate">{label}</p>
		</button>
	);
}
