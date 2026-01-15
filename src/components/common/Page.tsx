import type React from "react";

import type { PageType } from "~/types";
import { cn } from "~/util/cn";

interface PageProps {
	target: PageType;
	current: PageType;
	className?: string;
	children?: React.ReactNode;
}

export default function Page({ target, current, className, children }: PageProps) {
	return (
		<div
			className={cn(
				className,
				current === target ? "size-full flex flex-col overflow-auto" : "hidden",
			)}
		>
			{children}
		</div>
	);
}
