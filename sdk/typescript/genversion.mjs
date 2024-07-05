// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { readFile, writeFile } from 'fs/promises';
import TOML from '@iarna/toml';

const LICENSE = '// Copyright (c) Benfen\n// SPDX-License-Identifier: Apache-2.0\n\n';

const WARNING = '// This file is generated by genversion.mjs. Do not edit it directly.\n\n';

async function main() {
	const pkg = JSON.parse(await readFile('./package.json', 'utf8'));
	const cargo = TOML.parse(await readFile('../../Cargo.toml', 'utf8'));
	await writeFile(
		'src/version.ts',
		LICENSE +
			WARNING +
			`export const PACKAGE_VERSION = '${pkg.version}';\nexport const TARGETED_RPC_VERSION = '${cargo.workspace.package.version}';\n`,
	);
}

await main();
