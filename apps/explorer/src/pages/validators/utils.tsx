import { type ApyByValidator, formatPercentageDisplay } from '@mysten/core';
import { type SuiEvent, type SuiValidatorSummary, sui2ObcAddress } from '@mysten/sui.js';
import { Text } from '@mysten/ui';
import { StakeColumn } from '~/components/top-validators-card/StakeColumn';

import { ImageIcon } from '~/ui/ImageIcon';
import { Link } from '~/ui/Link';

import { Tooltip } from '~/ui/Tooltip';
import { ampli } from '~/utils/analytics/ampli';
import { getValidatorMoveEvent } from '~/utils/getValidatorMoveEvent';
import { VALIDATOR_LOW_STAKE_GRACE_PERIOD } from '~/utils/validatorConstants';

export function validatorsTableData(
	validators: SuiValidatorSummary[],
	atRiskValidators: [string, string][],
	validatorEvents: SuiEvent[],
	rollingAverageApys: ApyByValidator | null,
) {
	return {
		data: [...validators]
			.sort(() => 0.5 - Math.random())
			.map((validator, index) => {
				const validatorName = validator.name;
				const totalStake = validator.stakingPoolSuiBalance;
				const img = validator.imageUrl;

				const event = getValidatorMoveEvent(validatorEvents, validator.suiAddress);

				const atRiskValidator = atRiskValidators.find(
					([address]) => address === validator.suiAddress,
				);
				const isAtRisk = !!atRiskValidator;
				const lastReward = event?.pool_staking_reward ?? null;
				const { apy, isApyApproxZero } = rollingAverageApys?.[validator.suiAddress] ?? {
					apy: null,
				};
				const atRisk = isAtRisk
					? VALIDATOR_LOW_STAKE_GRACE_PERIOD - Number(atRiskValidator[1])
					: null;
				const label = 'At Risk';
				return {
					'#': (
						<Text variant="pBody/medium" color="steel-dark">
							{index + 1}
						</Text>
					),
					name: (
						<Link
							to={`/validator/${encodeURIComponent(sui2ObcAddress(validator.suiAddress))}`}
							onClick={() =>
								ampli.clickedValidatorRow({
									sourceFlow: 'Epoch details',
									validatorAddress: validator.suiAddress,
									validatorName: validatorName,
								})
							}
						>
							<div className="flex items-center gap-2.5">
								<ImageIcon
									src={validator.imageUrl}
									size="sm"
									label={validatorName}
									fallback={validatorName}
									circle
								/>
								<Text variant="pBody/medium" color="steel-darker">
									{validatorName}
								</Text>
							</div>
						</Link>
					),
					stake: <StakeColumn stake={totalStake} />,
					apy: (
						<Text variant="pBody/medium" color="steel-darker">
							{formatPercentageDisplay(apy, '--', isApyApproxZero)}
						</Text>
					),
					// {
					// 	apy,
					// 	isApyApproxZero,
					// },
					nextEpochGasPrice: <StakeColumn stake={validator.nextEpochGasPrice} inMIST />,
					commission: (
						<Text variant="pBody/medium" color="steel-darker">
							{Number(validator.commissionRate) / 100}%
						</Text>
					),
					img: img,
					address: validator.suiAddress,
					lastReward: lastReward ? (
						<StakeColumn stake={Number(lastReward)} />
					) : (
						<Text variant="pBody/medium" color="steel-darker">
							--
						</Text>
					),
					votingPower: (
						<Text variant="pBody/medium" color="steel-darker">
							{Number(validator.votingPower) / 100}%
						</Text>
					),
					atRisk:
						atRisk !== null ? (
							<Tooltip
								tip="Staked OBC is below the minimum OBC stake threshold to remain a validator."
								onOpen={() =>
									ampli.activatedTooltip({
										tooltipLabel: label,
									})
								}
							>
								<div className="flex cursor-pointer flex-nowrap items-center">
									<Text color="issue" variant="pBody/medium">
										{label}
									</Text>
									&nbsp;
									<Text uppercase variant="pBody/medium" color="steel-dark">
										{atRisk > 1 ? `in ${atRisk} epochs` : 'next epoch'}
									</Text>
								</div>
							</Tooltip>
						) : (
							<Text variant="pBody/medium" color="steel-darker">
								Active
							</Text>
						),
				};
			}),
		columns: [
            {
				header: '#',
				accessorKey: '#',
			},
			{
				header: 'Name',
				accessorKey: 'name',
			},
			{
				header: 'Stake',
				accessorKey: 'stake',
			},
			{
				header: 'Proposed Next Epoch Gas Price',
				accessorKey: 'nextEpochGasPrice',
			},
			{
				header: 'APY',
				accessorKey: 'apy',
			},
			{
				header: 'Commission',
				accessorKey: 'commission',
			},
			{
				header: 'Last Epoch Rewards',
				accessorKey: 'lastReward',
			},
			{
				header: 'Voting Power',
				accessorKey: 'votingPower',
			},
			{
				header: 'Status',
				accessorKey: 'atRisk',
			},
		],
	};
}

export function validatorsSortTableData(
	validators: SuiValidatorSummary[],
	atRiskValidators: [string, string][],
	validatorEvents: SuiEvent[],
	rollingAverageApys: ApyByValidator | null,
) {
	return {
		data: [...validators]
			.sort(() => 0.5 - Math.random())
			.map((validator) => {
				const validatorName = validator.name;
				const totalStake = validator.stakingPoolSuiBalance;
				const img = validator.imageUrl;

				const event = getValidatorMoveEvent(validatorEvents, validator.suiAddress);

				const atRiskValidator = atRiskValidators.find(
					([address]) => address === validator.suiAddress,
				);
				const isAtRisk = !!atRiskValidator;
				const lastReward = event?.pool_staking_reward ?? null;
				const { apy, isApyApproxZero } = rollingAverageApys?.[validator.suiAddress] ?? {
					apy: null,
				};

				return {
					name: {
						name: validatorName,
						logo: validator.imageUrl,
					},
					stake: totalStake,
					apy: {
						apy,
						isApyApproxZero,
					},
					nextEpochGasPrice: validator.nextEpochGasPrice,
					commission: Number(validator.commissionRate) / 100,
					img: img,
					address: validator.suiAddress,
					lastReward: lastReward ?? null,
					votingPower: Number(validator.votingPower) / 100,
					atRisk: isAtRisk ? VALIDATOR_LOW_STAKE_GRACE_PERIOD - Number(atRiskValidator[1]) : null,
				};
			}),
		columns: [
			{
				header: '#',
				accessorKey: 'number',
				cell: (props: any) => (
					<Text variant="pBody/medium" color="steel-dark">
						{props.table.getSortedRowModel().flatRows.indexOf(props.row) + 1}
					</Text>
				),
			},
			{
				header: 'Name',
				accessorKey: 'name',
				enableSorting: true,
				sortingFn: (a: any, b: any, colId: string) =>
					a.getValue(colId).name.localeCompare(b.getValue(colId).name, 'en', {
						sensitivity: 'base',
						numeric: true,
					}),
				cell: (props: any) => {
					const { name, logo } = props.getValue();
					return (
						<Link
							to={`/validator/${encodeURIComponent(sui2ObcAddress(props.row.original.address))}`}
							onClick={() =>
								ampli.clickedValidatorRow({
									sourceFlow: 'Epoch details',
									validatorAddress: props.row.original.address,
									validatorName: name,
								})
							}
						>
							<div className="flex items-center gap-2.5">
								<ImageIcon src={logo} size="sm" label={name} fallback={name} circle />
								<Text variant="pBody/medium" color="steel-darker">
									{name}
								</Text>
							</div>
						</Link>
					);
				},
			},
			{
				header: 'Stake',
				accessorKey: 'stake',
				enableSorting: true,
				cell: (props: any) => <StakeColumn stake={props.getValue()} />,
			},
			{
				header: 'Proposed Next Epoch Gas Price',
				accessorKey: 'nextEpochGasPrice',
				enableSorting: true,
				cell: (props: any) => <StakeColumn stake={props.getValue()} inMIST />,
			},
			{
				header: 'APY',
				accessorKey: 'apy',
				enableSorting: true,
				sortingFn: (a: any, b: any, colId: string) =>
					a.getValue(colId)?.apy < b.getValue(colId)?.apy ? -1 : 1,
				cell: (props: any) => {
					const { apy, isApyApproxZero } = props.getValue();
					return (
						<Text variant="pBody/medium" color="steel-darker">
							{formatPercentageDisplay(apy, '--', isApyApproxZero)}
						</Text>
					);
				},
			},
			{
				header: 'Commission',
				accessorKey: 'commission',
				enableSorting: true,
				cell: (props: any) => {
					const commissionRate = props.getValue();
					return (
						<Text variant="pBody/medium" color="steel-darker">
							{commissionRate}%
						</Text>
					);
				},
			},
			{
				header: 'Last Epoch Rewards',
				accessorKey: 'lastReward',
				enableSorting: true,
				cell: (props: any) => {
					const lastReward = props.getValue();
					return lastReward !== null ? (
						<StakeColumn stake={Number(lastReward)} />
					) : (
						<Text variant="pBody/medium" color="steel-darker">
							--
						</Text>
					);
				},
			},
			{
				header: 'Voting Power',
				accessorKey: 'votingPower',
				enableSorting: true,
				cell: (props: any) => {
					const votingPower = props.getValue();
					return (
						<Text variant="pBody/medium" color="steel-darker">
							{votingPower}%
						</Text>
					);
				},
			},
			{
				header: 'Status',
				accessorKey: 'atRisk',
				cell: (props: any) => {
					const atRisk = props.getValue();
					const label = 'At Risk';
					return atRisk !== null ? (
						<Tooltip
							tip="Staked OBC is below the minimum OBC stake threshold to remain a validator."
							onOpen={() =>
								ampli.activatedTooltip({
									tooltipLabel: label,
								})
							}
						>
							<div className="flex cursor-pointer flex-nowrap items-center">
								<Text color="issue" variant="pBody/medium">
									{label}
								</Text>
								&nbsp;
								<Text uppercase variant="pBody/medium" color="steel-dark">
									{atRisk > 1 ? `in ${atRisk} epochs` : 'next epoch'}
								</Text>
							</div>
						</Tooltip>
					) : (
						<Text variant="pBody/medium" color="steel-darker">
							Active
						</Text>
					);
				},
			},
		],
	};
}
