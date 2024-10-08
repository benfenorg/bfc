// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { sui2BfcAddress } from '@benfen/bfc.js';
import { type SuiSystemStateSummary } from '@benfen/bfc.js/client';
import { useGetValidatorsApy, useGetValidatorsEvents } from '@mysten/core';
import { useLatestSuiSystemState } from '@benfen/bfc.js/dapp-kit';
import { LoadingIndicator } from '@mysten/ui';
import React, { useMemo } from 'react';
import { useParams } from 'react-router-dom';

import { PageLayout } from '~/components/Layout/PageLayout';
import { ValidatorMeta } from '~/components/validator/ValidatorMeta';
import { ValidatorStats } from '~/components/validator/ValidatorStats';
import { Banner } from '~/ui/Banner';
import { getValidatorMoveEvent } from '~/utils/getValidatorMoveEvent';
import { VALIDATOR_LOW_STAKE_GRACE_PERIOD } from '~/utils/validatorConstants';

const getAtRiskRemainingEpochs = (
	data: SuiSystemStateSummary | undefined,
	validatorId: string | undefined,
): number | null => {
	if (!data || !validatorId) return null;
	const atRisk = data.atRiskValidators.find(([address]) => address === validatorId);
	return atRisk ? VALIDATOR_LOW_STAKE_GRACE_PERIOD - Number(atRisk[1]) : null;
};

function ValidatorDetails() {
	const { id } = useParams();
	const { data, isLoading } = useLatestSuiSystemState();

	const validatorData = useMemo(() => {
		if (!data) return null;
		return (
			data.activeValidators.find(
				({ suiAddress, stakingPoolId }) =>
					sui2BfcAddress(suiAddress) === id || sui2BfcAddress(stakingPoolId) === id,
			) || null
		);
	}, [id, data]);
	const atRiskRemainingEpochs = getAtRiskRemainingEpochs(data, id);
	console.log(atRiskRemainingEpochs);

	const numberOfValidators = data?.activeValidators.length ?? null;
	const { data: rollingAverageApys, isLoading: validatorsApysLoading } = useGetValidatorsApy();

	const { data: validatorEvents, isLoading: validatorsEventsLoading } = useGetValidatorsEvents({
		limit: numberOfValidators,
		order: 'descending',
	});

	const validatorRewards = useMemo(() => {
		if (!validatorEvents || !id) return 0;
		const rewards = (getValidatorMoveEvent(validatorEvents, id) as { pool_staking_reward: string })
			?.pool_staking_reward;

		return rewards ? Number(rewards) : null;
	}, [id, validatorEvents]);

	if (isLoading || validatorsEventsLoading || validatorsApysLoading) {
		return (
			<PageLayout
				content={
					<div className="mb-10 flex items-center justify-center">
						<LoadingIndicator />
					</div>
				}
			/>
		);
	}

	if (!validatorData || !data || !validatorEvents || !id) {
		return (
			<PageLayout
				content={
					<div className="mb-10 flex items-center justify-center">
						<LoadingIndicator />
					</div>
				}
			/>
		);
	}

	if (!validatorData || !data || !validatorEvents || !id) {
		return (
			<PageLayout
				content={
					<div className="mb-10 flex items-center justify-center">
						<Banner variant="error" spacing="lg" fullWidth>
							No validator data found for {id}
						</Banner>
					</div>
				}
			/>
		);
	}
	const { apy, isApyApproxZero } = rollingAverageApys?.[id] ?? { apy: null };

	const tallyingScore =
		(
			validatorEvents as {
				parsedJson?: { tallying_rule_global_score?: string; validator_address?: string };
			}[]
		)?.find(({ parsedJson }) => parsedJson?.validator_address === id)?.parsedJson
			?.tallying_rule_global_score || null;

	return (
		<PageLayout
			content={
				<div className="mb-10 grid grid-cols-2 gap-8">
					<div className="flex flex-col gap-1 md:gap-0">
						<ValidatorMeta validatorData={validatorData} />
					</div>
					<div className="rounded-xl border border-solid border-bfc-border p-7.5">
						<ValidatorStats
							validatorData={validatorData}
							epoch={data.epoch}
							epochRewards={validatorRewards}
							apy={isApyApproxZero ? '~0' : apy}
							tallyingScore={tallyingScore}
						/>
					</div>
				</div>
			}
		/>
	);
}

export { ValidatorDetails };
