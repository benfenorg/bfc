// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_src/ui/app/shared/text';
import { ChevronDown12, ChevronRight12 } from '@mysten/icons';
import { useState } from 'react';

interface CommandProps {
	command: any;
}

export function Command({ command }: CommandProps) {
	const [expanded, setExpanded] = useState(true);

	return (
		<div>
			<button
				onClick={() => setExpanded((expanded) => !expanded)}
				className="flex items-center gap-2 w-full bg-transparent border-none p-0"
			>
				<Text variant="body" weight="semibold" color="steel-darker">
					{command.kind}
				</Text>
				<div className="h-px bg-gray-40 flex-1" />
				<div className="text-steel">{expanded ? <ChevronDown12 /> : <ChevronRight12 />}</div>
			</button>

			{expanded && (
				<div className="mt-2 text-pBodySmall font-medium text-steel">{JSON.stringify(command)}</div>
			)}
		</div>
	);
}
