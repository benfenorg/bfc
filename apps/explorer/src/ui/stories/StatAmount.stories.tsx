// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type Meta, type StoryObj } from '@storybook/react';

import { StatAmount, type StatAmountProps } from '../StatAmount';

export default {
	component: StatAmount,
} as Meta;

export const defaultAmount: StoryObj<StatAmountProps> = {
<<<<<<< Updated upstream
	args: {
		amount: 9740991,
		symbol: 'SUI',
		dollarAmount: 123.56,
		date: 1667942429177,
	},
=======
    args: {
        amount: 9740991,
        symbol: 'OBC',
        dollarAmount: 123.56,
        date: 1667942429177,
    },
>>>>>>> Stashed changes
};
