import CommandItem from "~/components/commands/CommandItem";
import Expander from "~/components/common/Expander";
import Page from "~/components/common/Page";
import { useConfigStore } from "~/stores/config";
import { PageType } from "~/types";

interface HomeProps {
	page: PageType;
	search: string;
}

export default function Commands({ page, search }: HomeProps) {
	const commands = useConfigStore((s) => s.commands);

	const filteredCommandGroups = Object.entries(commands)
		.map(([category, commands]) => ({
			category,
			commands: commands.filter((command) =>
				command.label.toLowerCase().includes(search.toLowerCase()),
			),
		}))
		.filter((group) => group.commands.length > 0);

	return (
		<Page
			target={PageType.COMMANDS}
			current={page}
			className="gap-3"
		>
			{filteredCommandGroups.length !== 0 &&
				filteredCommandGroups.map((group) => (
					<Expander
						label={group.category}
						key={group.category}
					>
						{group.commands.map((command) => (
							<CommandItem
								key={command.label}
								command={command}
							/>
						))}
					</Expander>
				))}
		</Page>
	);
}
