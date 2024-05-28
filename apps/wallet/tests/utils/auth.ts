// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Page } from '@playwright/test';

export const PASSWORD = 'mystenlabs';

export async function createWallet(page: Page, extensionUrl: string) {
	await page.goto(extensionUrl);
	await page.getByRole('link', { name: /More Options/ }).click();
	await page.getByRole('link', { name: /Create a new Passphrase Account/ }).click();
	await page.getByLabel('Create Account Password').fill('mystenlabs');
	await page.getByLabel('Confirm Account Password').fill('mystenlabs');
	await page.getByLabel('I read and agreed to the').click();
	await page.getByRole('button', { name: /Create Wallet/ }).click();
	await page.locator('label', { has: page.locator('input[type=checkbox]') }).click();
<<<<<<< HEAD
	await page.getByRole('link', { name: /Open BenFen Wallet/ }).click();
	await page.getByTestId('bullshark-dismiss').click();
=======
	await page.getByRole('link', { name: /Open Sui Wallet/ }).click();
>>>>>>> mainnet-v1.24.1
}

export async function importWallet(page: Page, extensionUrl: string, mnemonic: string | string[]) {
	await page.goto(extensionUrl);
<<<<<<< HEAD
	await page.getByRole('link', { name: /Get Started/ }).click();
	await page.getByRole('link', { name: /Import an Existing Wallet/ }).click();
	const inputs = await page.locator('input[type=password]');
	const inputsCount = await inputs.count();
	for (let i = 0; i < inputsCount; i++) {
		await inputs.nth(i).fill(mnemonic[i]);
	}
	await page.getByRole('button', { name: /Continue/ }).click();
	await page.getByLabel('Create Password').fill(PASSWORD);
	await page.getByLabel('Confirm Password').fill(PASSWORD);
	await page.getByRole('button', { name: /Import/ }).click();
	await page.getByRole('link', { name: /Open BenFen Wallet/ }).click();
	await page.getByTestId('bullshark-dismiss').click();
=======
	await page.getByRole('link', { name: /More Options/ }).click();
	await page.getByRole('link', { name: /Import Passphrase/ }).click();
	await page
		.getByPlaceholder('Password')
		.first()
		.type(typeof mnemonic === 'string' ? mnemonic : mnemonic.join(' '));
	await page.getByRole('button', { name: /Add Account/ }).click();
	await page.getByLabel('Create Account Password').fill(PASSWORD);
	await page.getByLabel('Confirm Account Password').fill(PASSWORD);
	await page.getByLabel('I read and agreed to the').click();
	await page.getByRole('button', { name: /Create Wallet/ }).click();
>>>>>>> mainnet-v1.24.1
}
