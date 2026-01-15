import { useEffect, useState } from "react";

import Apps from "~/pages/Apps";
import Commands from "~/pages/Commands";
import Home from "~/pages/Home";

import Header from "./components/layout/Header";
import Navigation from "./components/layout/Navigation";
import { PageType } from "./types";

export default function App() {
	const [page, setPage] = useState<PageType>(PageType.HOME);
	const [search, setSearch] = useState<string>("");

	useEffect(() => {
		const onContextMenu = (e: MouseEvent) => {
			if (!import.meta.env.DEV) {
				e.preventDefault();
			}
		};

		document.addEventListener("contextmenu", onContextMenu);

		return () => {
			document.removeEventListener("contextmenu", onContextMenu);
		};
	}, []);

	return (
		<div className="w-screen h-screen flex flex-col overflow-hidden">
			<Header setPage={setPage} />
			<Navigation
				page={page}
				setPage={setPage}
				search={search}
				setSearch={setSearch}
			/>

			<Home page={page} />
			<Apps
				page={page}
				search={search}
			/>
			<Commands
				page={page}
				search={search}
			/>
		</div>
	);
}
