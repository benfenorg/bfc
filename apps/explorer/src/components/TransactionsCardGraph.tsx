// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAmount, formatDate, useGetTotalTransactionBlocks, useRpcClient } from '@mysten/core';
import { Heading, Text, LoadingIndicator } from '@mysten/ui';
import { useQuery } from '@tanstack/react-query';
import { ParentSize } from '@visx/responsive';
import clsx from 'clsx';

import { AreaGraph } from './AreaGraph';
import { ErrorBoundary } from './error-boundary/ErrorBoundary';
import { Card } from '~/ui/Card';

function TooltipContent({
	data: { epochTotalTransactions, epochStartTimestamp, epoch },
}: {
	data: {
		epochTotalTransactions: number;
		epochStartTimestamp: number;
		epoch: number;
	};
}) {
	const dateFormatted = formatDate(new Date(epochStartTimestamp), ['day', 'month']);
	const totalFormatted = formatAmount(epochTotalTransactions);
	return (
		<div className="flex flex-col gap-0.5">
			<Text variant="subtitleSmallExtra/medium" color="steel-darker">
				{dateFormatted}, Epoch {epoch}
			</Text>
			<Heading variant="heading6/semibold" color="steel-darker">
				{totalFormatted}
			</Heading>
			<Text variant="subtitleSmallExtra/medium" color="steel-darker" uppercase>
				Transaction Blocks
			</Text>
		</div>
	);
}
const aaa = [
    {
        "epoch": "133",
        "validators": [],
        "epochTotalTransactions": "1744461",
        "firstCheckpointId": "11110104",
        "epochStartTimestamp": "1692810578523",
        "endOfEpochInfo": {
            "lastCheckpointId": "11194784",
            "epochEndTimestamp": "1692896982617",
            "protocolVersion": "20",
            "referenceGasPrice": "755",
            "totalStake": "7433791622348449042",
            "storageFundReinvestment": "3149407",
            "storageCharge": "31450880082000",
            "storageRebate": "29242761711768",
            "storageFundBalance": "128108247537639",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3722947449768",
            "totalStakeRewardsDistributed": "1003722944300233",
            "leftoverStorageFundInflow": "128"
        },
        "referenceGasPrice": 755
    },
    {
        "epoch": "132",
        "validators": [],
        "epochTotalTransactions": "1737013",
        "firstCheckpointId": "11025134",
        "epochStartTimestamp": "1692724174555",
        "endOfEpochInfo": {
            "lastCheckpointId": "11110103",
            "epochEndTimestamp": "1692810578523",
            "protocolVersion": "20",
            "referenceGasPrice": "755",
            "totalStake": "7441267469794639787",
            "storageFundReinvestment": "3204071",
            "storageCharge": "32641093616400",
            "storageRebate": "30608902374732",
            "storageFundBalance": "125900126017872",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3841402901384",
            "totalStakeRewardsDistributed": "1003841399697201",
            "leftoverStorageFundInflow": "112"
        },
        "referenceGasPrice": 755
    },
    {
        "epoch": "131",
        "validators": [],
        "epochTotalTransactions": "1611236",
        "firstCheckpointId": "10940241",
        "epochStartTimestamp": "1692637770317",
        "endOfEpochInfo": {
            "lastCheckpointId": "11025133",
            "epochEndTimestamp": "1692724174555",
            "protocolVersion": "20",
            "referenceGasPrice": "755",
            "totalStake": "7425218755837970393",
            "storageFundReinvestment": "3261341",
            "storageCharge": "28895050044400",
            "storageRebate": "26846270772552",
            "storageFundBalance": "123867931572021",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3975584940656",
            "totalStakeRewardsDistributed": "1003975581679199",
            "leftoverStorageFundInflow": "116"
        },
        "referenceGasPrice": 755
    },
    {
        "epoch": "130",
        "validators": [],
        "epochTotalTransactions": "1656716",
        "firstCheckpointId": "10854305",
        "epochStartTimestamp": "1692551365967",
        "endOfEpochInfo": {
            "lastCheckpointId": "10940240",
            "epochEndTimestamp": "1692637770317",
            "protocolVersion": "20",
            "referenceGasPrice": "759",
            "totalStake": "7424772421379870327",
            "storageFundReinvestment": "3261592",
            "storageCharge": "27109551097200",
            "storageRebate": "25269963760008",
            "storageFundBalance": "121819149038716",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "4047008173696",
            "totalStakeRewardsDistributed": "1004047004911957",
            "leftoverStorageFundInflow": "147"
        },
        "referenceGasPrice": 759
    },
    {
        "epoch": "129",
        "validators": [],
        "epochTotalTransactions": "2406973",
        "firstCheckpointId": "10768424",
        "epochStartTimestamp": "1692464961541",
        "endOfEpochInfo": {
            "lastCheckpointId": "10854304",
            "epochEndTimestamp": "1692551365967",
            "protocolVersion": "20",
            "referenceGasPrice": "760",
            "totalStake": "7443457324371674148",
            "storageFundReinvestment": "3640107",
            "storageCharge": "31183950358800",
            "storageRebate": "27976293109512",
            "storageFundBalance": "119979558439785",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "4644034496616",
            "totalStakeRewardsDistributed": "1004644030856441",
            "leftoverStorageFundInflow": "68"
        },
        "referenceGasPrice": 760
    },
    {
        "epoch": "128",
        "validators": [],
        "epochTotalTransactions": "2300258",
        "firstCheckpointId": "10682553",
        "epochStartTimestamp": "1692378556646",
        "endOfEpochInfo": {
            "lastCheckpointId": "10768423",
            "epochEndTimestamp": "1692464961541",
            "protocolVersion": "20",
            "referenceGasPrice": "765",
            "totalStake": "7448738193900346678",
            "storageFundReinvestment": "3577653",
            "storageCharge": "33154013398400",
            "storageRebate": "32557931809524",
            "storageFundBalance": "116771897550322",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "4586515166908",
            "totalStakeRewardsDistributed": "1004586511589098",
            "leftoverStorageFundInflow": "157"
        },
        "referenceGasPrice": 765
    },
    {
        "epoch": "127",
        "validators": [],
        "epochTotalTransactions": "2224649",
        "firstCheckpointId": "10598016",
        "epochStartTimestamp": "1692292150718",
        "endOfEpochInfo": {
            "lastCheckpointId": "10682552",
            "epochEndTimestamp": "1692378556646",
            "protocolVersion": "20",
            "referenceGasPrice": "765",
            "totalStake": "7446693242267612942",
            "storageFundReinvestment": "3056569",
            "storageCharge": "35541673896400",
            "storageRebate": "38011897983312",
            "storageFundBalance": "116175812383636",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3832326926464",
            "totalStakeRewardsDistributed": "1003832323869778",
            "leftoverStorageFundInflow": "117"
        },
        "referenceGasPrice": 765
    },
    {
        "epoch": "126",
        "validators": [],
        "epochTotalTransactions": "2013050",
        "firstCheckpointId": "10513354",
        "epochStartTimestamp": "1692205746798",
        "endOfEpochInfo": {
            "lastCheckpointId": "10598015",
            "epochEndTimestamp": "1692292150718",
            "protocolVersion": "20",
            "referenceGasPrice": "765",
            "totalStake": "7437800674887300328",
            "storageFundReinvestment": "2776825",
            "storageCharge": "27926722059200",
            "storageRebate": "29635910123688",
            "storageFundBalance": "118646033413862",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3437574113000",
            "totalStakeRewardsDistributed": "1003437571336122",
            "leftoverStorageFundInflow": "53"
        },
        "referenceGasPrice": 765
    },
    {
        "epoch": "125",
        "validators": [],
        "epochTotalTransactions": "1723059",
        "firstCheckpointId": "10428013",
        "epochStartTimestamp": "1692119342751",
        "endOfEpochInfo": {
            "lastCheckpointId": "10513353",
            "epochEndTimestamp": "1692205746798",
            "protocolVersion": "19",
            "referenceGasPrice": "765",
            "totalStake": "7449574275280456567",
            "storageFundReinvestment": "2632700",
            "storageCharge": "25061399980400",
            "storageRebate": "24755029619364",
            "storageFundBalance": "120355218701472",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3266815278584",
            "totalStakeRewardsDistributed": "1003266812645781",
            "leftoverStorageFundInflow": "103"
        },
        "referenceGasPrice": 765
    },
    {
        "epoch": "124",
        "validators": [],
        "epochTotalTransactions": "1540116",
        "firstCheckpointId": "10341484",
        "epochStartTimestamp": "1692032939601",
        "endOfEpochInfo": {
            "lastCheckpointId": "10428012",
            "epochEndTimestamp": "1692119342751",
            "protocolVersion": "19",
            "referenceGasPrice": "765",
            "totalStake": "7448076270451084035",
            "storageFundReinvestment": "2673162",
            "storageCharge": "23247284304800",
            "storageRebate": "20841903044424",
            "storageFundBalance": "120048845707633",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3384434055320",
            "totalStakeRewardsDistributed": "1003384431382088",
            "leftoverStorageFundInflow": "70"
        },
        "referenceGasPrice": 765
    },
    {
        "epoch": "123",
        "validators": [],
        "epochTotalTransactions": "1612636",
        "firstCheckpointId": "10254488",
        "epochStartTimestamp": "1691946535386",
        "endOfEpochInfo": {
            "lastCheckpointId": "10341483",
            "epochEndTimestamp": "1692032939601",
            "protocolVersion": "19",
            "referenceGasPrice": "777",
            "totalStake": "7447176366944471214",
            "storageFundReinvestment": "2702889",
            "storageCharge": "24299661860400",
            "storageRebate": "28194921423504",
            "storageFundBalance": "117643461774025",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3311862158860",
            "totalStakeRewardsDistributed": "1003311859455864",
            "leftoverStorageFundInflow": "107"
        },
        "referenceGasPrice": 777
    },
    {
        "epoch": "122",
        "validators": [],
        "epochTotalTransactions": "1625533",
        "firstCheckpointId": "10167634",
        "epochStartTimestamp": "1691860131132",
        "endOfEpochInfo": {
            "lastCheckpointId": "10254487",
            "epochEndTimestamp": "1691946535386",
            "protocolVersion": "19",
            "referenceGasPrice": "780",
            "totalStake": "7445973258337374393",
            "storageFundReinvestment": "2805469",
            "storageCharge": "23845295016800",
            "storageRebate": "21613878084744",
            "storageFundBalance": "121538718634133",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3501291338928",
            "totalStakeRewardsDistributed": "1003501288533372",
            "leftoverStorageFundInflow": "87"
        },
        "referenceGasPrice": 780
    },
    {
        "epoch": "121",
        "validators": [],
        "epochTotalTransactions": "1508040",
        "firstCheckpointId": "10081676",
        "epochStartTimestamp": "1691773726434",
        "endOfEpochInfo": {
            "lastCheckpointId": "10167633",
            "epochEndTimestamp": "1691860131132",
            "protocolVersion": "19",
            "referenceGasPrice": "780",
            "totalStake": "7444795918927160708",
            "storageFundReinvestment": "2587317",
            "storageCharge": "30012454606000",
            "storageRebate": "27810114719904",
            "storageFundBalance": "119307298896521",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3289368623544",
            "totalStakeRewardsDistributed": "1003289366036137",
            "leftoverStorageFundInflow": "90"
        },
        "referenceGasPrice": 780
    },
    {
        "epoch": "120",
        "validators": [],
        "epochTotalTransactions": "1267048",
        "firstCheckpointId": "9994980",
        "epochStartTimestamp": "1691687321866",
        "endOfEpochInfo": {
            "lastCheckpointId": "10081675",
            "epochEndTimestamp": "1691773726434",
            "protocolVersion": "19",
            "referenceGasPrice": "781",
            "totalStake": "7443912576869433053",
            "storageFundReinvestment": "2282243",
            "storageCharge": "22857213119200",
            "storageRebate": "20757166409628",
            "storageFundBalance": "117104956423018",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2951282697192",
            "totalStakeRewardsDistributed": "1002951280414846",
            "leftoverStorageFundInflow": "103"
        },
        "referenceGasPrice": 781
    },
    {
        "epoch": "119",
        "validators": [],
        "epochTotalTransactions": "1430268",
        "firstCheckpointId": "9908937",
        "epochStartTimestamp": "1691600917211",
        "endOfEpochInfo": {
            "lastCheckpointId": "9994979",
            "epochEndTimestamp": "1691687321866",
            "protocolVersion": "19",
            "referenceGasPrice": "780",
            "totalStake": "7435812665686334381",
            "storageFundReinvestment": "2335295",
            "storageCharge": "22597985540000",
            "storageRebate": "23064252230052",
            "storageFundBalance": "115004907431100",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3006857028000",
            "totalStakeRewardsDistributed": "1003006854692622",
            "leftoverStorageFundInflow": "83"
        },
        "referenceGasPrice": 780
    },
    {
        "epoch": "118",
        "validators": [],
        "epochTotalTransactions": "1355460",
        "firstCheckpointId": "9822168",
        "epochStartTimestamp": "1691514513223",
        "endOfEpochInfo": {
            "lastCheckpointId": "9908936",
            "epochEndTimestamp": "1691600917211",
            "protocolVersion": "19",
            "referenceGasPrice": "781",
            "totalStake": "7433747890094097365",
            "storageFundReinvestment": "2268837",
            "storageCharge": "23542790886800",
            "storageRebate": "21430758115008",
            "storageFundBalance": "115471171785774",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2973646393616",
            "totalStakeRewardsDistributed": "1002973644124674",
            "leftoverStorageFundInflow": "105"
        },
        "referenceGasPrice": 781
    },
    {
        "epoch": "117",
        "validators": [],
        "epochTotalTransactions": "1136018",
        "firstCheckpointId": "9735353",
        "epochStartTimestamp": "1691428108633",
        "endOfEpochInfo": {
            "lastCheckpointId": "9822167",
            "epochEndTimestamp": "1691514513223",
            "protocolVersion": "19",
            "referenceGasPrice": "790",
            "totalStake": "7428581775388490688",
            "storageFundReinvestment": "2119634",
            "storageCharge": "22515181883200",
            "storageRebate": "21642801424200",
            "storageFundBalance": "113359136745040",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2795189735008",
            "totalStakeRewardsDistributed": "1002795187615225",
            "leftoverStorageFundInflow": "149"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "116",
        "validators": [],
        "epochTotalTransactions": "1004159",
        "firstCheckpointId": "9648292",
        "epochStartTimestamp": "1691341704745",
        "endOfEpochInfo": {
            "lastCheckpointId": "9735352",
            "epochEndTimestamp": "1691428108633",
            "protocolVersion": "19",
            "referenceGasPrice": "782",
            "totalStake": "7416774007596541284",
            "storageFundReinvestment": "2203455",
            "storageCharge": "20588591998800",
            "storageRebate": "26775622579464",
            "storageFundBalance": "112486754166257",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2753454516000",
            "totalStakeRewardsDistributed": "1002753452312449",
            "leftoverStorageFundInflow": "96"
        },
        "referenceGasPrice": 782
    },
    {
        "epoch": "115",
        "validators": [],
        "epochTotalTransactions": "865281",
        "firstCheckpointId": "9561270",
        "epochStartTimestamp": "1691255300122",
        "endOfEpochInfo": {
            "lastCheckpointId": "9648291",
            "epochEndTimestamp": "1691341704745",
            "protocolVersion": "19",
            "referenceGasPrice": "785",
            "totalStake": "7414665252387456186",
            "storageFundReinvestment": "2368219",
            "storageCharge": "19626657338800",
            "storageRebate": "25281877494852",
            "storageFundBalance": "118673782543370",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2823577062504",
            "totalStakeRewardsDistributed": "1002823574694145",
            "leftoverStorageFundInflow": "140"
        },
        "referenceGasPrice": 785
    },
    {
        "epoch": "114",
        "validators": [],
        "epochTotalTransactions": "1289748",
        "firstCheckpointId": "9474153",
        "epochStartTimestamp": "1691168896905",
        "endOfEpochInfo": {
            "lastCheckpointId": "9561269",
            "epochEndTimestamp": "1691255300122",
            "protocolVersion": "19",
            "referenceGasPrice": "785",
            "totalStake": "7411616050010855437",
            "storageFundReinvestment": "2339027",
            "storageCharge": "32720086276000",
            "storageRebate": "32363977941900",
            "storageFundBalance": "124329000331063",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2796263453784",
            "totalStakeRewardsDistributed": "1002796261114653",
            "leftoverStorageFundInflow": "104"
        },
        "referenceGasPrice": 785
    },
    {
        "epoch": "113",
        "validators": [],
        "epochTotalTransactions": "1386292",
        "firstCheckpointId": "9387110",
        "epochStartTimestamp": "1691082492182",
        "endOfEpochInfo": {
            "lastCheckpointId": "9474152",
            "epochEndTimestamp": "1691168896905",
            "protocolVersion": "19",
            "referenceGasPrice": "790",
            "totalStake": "7410240647767754209",
            "storageFundReinvestment": "2393565",
            "storageCharge": "33638326058400",
            "storageRebate": "33108114475656",
            "storageFundBalance": "123972889657832",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2873479117680",
            "totalStakeRewardsDistributed": "1002873476724016",
            "leftoverStorageFundInflow": "99"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "112",
        "validators": [],
        "epochTotalTransactions": "1428387",
        "firstCheckpointId": "9300671",
        "epochStartTimestamp": "1690996087446",
        "endOfEpochInfo": {
            "lastCheckpointId": "9387109",
            "epochEndTimestamp": "1691082492182",
            "protocolVersion": "19",
            "referenceGasPrice": "785",
            "totalStake": "7409534115435237579",
            "storageFundReinvestment": "2392732",
            "storageCharge": "37301118462800",
            "storageRebate": "35860150003392",
            "storageFundBalance": "123442675681424",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2905960456176",
            "totalStakeRewardsDistributed": "1002905958063336",
            "leftoverStorageFundInflow": "108"
        },
        "referenceGasPrice": 785
    },
    {
        "epoch": "111",
        "validators": [],
        "epochTotalTransactions": "1499516",
        "firstCheckpointId": "9214069",
        "epochStartTimestamp": "1690909683171",
        "endOfEpochInfo": {
            "lastCheckpointId": "9300670",
            "epochEndTimestamp": "1690996087446",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7408399452263109773",
            "storageFundReinvestment": "2352997",
            "storageCharge": "37769611054000",
            "storageRebate": "36060186934728",
            "storageFundBalance": "122001704829176",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "2900659266016",
            "totalStakeRewardsDistributed": "1002900656912908",
            "leftoverStorageFundInflow": "111"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "110",
        "validators": [],
        "epochTotalTransactions": "1922618",
        "firstCheckpointId": "9127065",
        "epochStartTimestamp": "1690823278788",
        "endOfEpochInfo": {
            "lastCheckpointId": "9214068",
            "epochEndTimestamp": "1690909683171",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7414398712555230261",
            "storageFundReinvestment": "2589969",
            "storageCharge": "47048913301200",
            "storageRebate": "43799717332008",
            "storageFundBalance": "120292278356796",
            "stakeSubsidyAmount": "1000000000000000",
            "totalGasFees": "3280880745200",
            "totalStakeRewardsDistributed": "1003280878155155",
            "leftoverStorageFundInflow": "76"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "109",
        "validators": [],
        "epochTotalTransactions": "2589687",
        "firstCheckpointId": "9039836",
        "epochStartTimestamp": "1690736874468",
        "endOfEpochInfo": {
            "lastCheckpointId": "9127064",
            "epochEndTimestamp": "1690823278788",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7413182928838371129",
            "storageFundReinvestment": "2924685",
            "storageCharge": "63100080043600",
            "storageRebate": "55707648628896",
            "storageFundBalance": "117043079797559",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "3953960584600",
            "totalStakeRewardsDistributed": "1115065068770883",
            "leftoverStorageFundInflow": "143"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "108",
        "validators": [],
        "epochTotalTransactions": "2285902",
        "firstCheckpointId": "8952830",
        "epochStartTimestamp": "1690650470203",
        "endOfEpochInfo": {
            "lastCheckpointId": "9039835",
            "epochEndTimestamp": "1690736874468",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7411871425939808009",
            "storageFundReinvestment": "2589429",
            "storageCharge": "56821166566000",
            "storageRebate": "48719884989204",
            "storageFundBalance": "109650645458027",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "3779311556000",
            "totalStakeRewardsDistributed": "1114890420077574",
            "leftoverStorageFundInflow": "108"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "107",
        "validators": [],
        "epochTotalTransactions": "2296443",
        "firstCheckpointId": "8865935",
        "epochStartTimestamp": "1690564065790",
        "endOfEpochInfo": {
            "lastCheckpointId": "8952829",
            "epochEndTimestamp": "1690650470203",
            "protocolVersion": "18",
            "referenceGasPrice": "791",
            "totalStake": "7410538937670632551",
            "storageFundReinvestment": "2431176",
            "storageCharge": "51964706883200",
            "storageRebate": "44781108684444",
            "storageFundBalance": "101549361291694",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "3792487692800",
            "totalStakeRewardsDistributed": "1114903596372664",
            "leftoverStorageFundInflow": "71"
        },
        "referenceGasPrice": 791
    },
    {
        "epoch": "106",
        "validators": [],
        "epochTotalTransactions": "39578614",
        "firstCheckpointId": "8778860",
        "epochStartTimestamp": "1690477660979",
        "endOfEpochInfo": {
            "lastCheckpointId": "8865934",
            "epochEndTimestamp": "1690564065790",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7360148193538852384",
            "storageFundReinvestment": "19988188",
            "storageCharge": "158588190385600",
            "storageRebate": "153214641679968",
            "storageFundBalance": "94365760661691",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "33023670601800",
            "totalStakeRewardsDistributed": "1144134761724619",
            "leftoverStorageFundInflow": "104"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "105",
        "validators": [],
        "epochTotalTransactions": "65103934",
        "firstCheckpointId": "8692745",
        "epochStartTimestamp": "1690391255635",
        "endOfEpochInfo": {
            "lastCheckpointId": "8778859",
            "epochEndTimestamp": "1690477660979",
            "protocolVersion": "18",
            "referenceGasPrice": "791",
            "totalStake": "7351374693820953485",
            "storageFundReinvestment": "29797621",
            "storageCharge": "238543357123200",
            "storageRebate": "232632645362964",
            "storageFundBalance": "88992191967767",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "53079501179000",
            "totalStakeRewardsDistributed": "1164190582492380",
            "leftoverStorageFundInflow": "110"
        },
        "referenceGasPrice": 791
    },
    {
        "epoch": "104",
        "validators": [],
        "epochTotalTransactions": "60599925",
        "firstCheckpointId": "8607838",
        "epochStartTimestamp": "1690304848221",
        "endOfEpochInfo": {
            "lastCheckpointId": "8692744",
            "epochEndTimestamp": "1690391255635",
            "protocolVersion": "18",
            "referenceGasPrice": "790",
            "totalStake": "7399705500191810356",
            "storageFundReinvestment": "25897160",
            "storageCharge": "225682741478800",
            "storageRebate": "219863169146412",
            "storageFundBalance": "83081450409800",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "49617788238000",
            "totalStakeRewardsDistributed": "1160728873451820",
            "leftoverStorageFundInflow": "131"
        },
        "referenceGasPrice": 790
    },
    {
        "epoch": "103",
        "validators": [],
        "epochTotalTransactions": "64614127",
        "firstCheckpointId": "8522891",
        "epochStartTimestamp": "1690218442397",
        "endOfEpochInfo": {
            "lastCheckpointId": "8607837",
            "epochEndTimestamp": "1690304848221",
            "protocolVersion": "18",
            "referenceGasPrice": "791",
            "totalStake": "7401433541409315298",
            "storageFundReinvestment": "25727723",
            "storageCharge": "222896546237600",
            "storageRebate": "217972678297572",
            "storageFundBalance": "77261852180121",
            "stakeSubsidyAmount": "1111111111111111",
            "totalGasFees": "52640291075000",
            "totalStakeRewardsDistributed": "1163751376458233",
            "leftoverStorageFundInflow": "155"
        },
        "referenceGasPrice": 791
    }
]
function useEpochTransactions() {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['get', 'last', '30', 'epoch', 'transactions'],
		queryFn: async () =>
			[
				...(
					await rpc.getEpochs({
						descendingOrder: true,
						limit: 31,
					})
				).data,
				// ...aaa
			]
				.reverse()
				.slice(0, -1),
		select: (data) =>
			data.map(({ epoch, epochTotalTransactions, epochStartTimestamp }) => ({
				epoch: Number(epoch),
				epochTotalTransactions: Number(epochTotalTransactions),
				epochStartTimestamp: Number(epochStartTimestamp),
			})),
	});
}

export function TransactionsCardGraph() {
	const { data: epochMetrics, isLoading } = useEpochTransactions();

	return (
		<Card bg="white" spacing={!epochMetrics?.length ? 'lg' : 'lgGraph'} height="full">
			<div className="flex h-full flex-col gap-4 overflow-hidden">
				<div className="heading6 text-[#171719] font-bold">
					Transaction Blocks
				</div>
				<div
					className={clsx(
						'flex min-h-[180px] flex-1 flex-col items-center justify-center rounded-xl transition-colors',
						!epochMetrics?.length && 'bg-gray-40',
					)}
				>
					{isLoading ? (
						<div className="flex flex-col items-center gap-1">
							<LoadingIndicator />
							<Text color="steel" variant="body/medium">
								loading data
							</Text>
						</div>
					) : epochMetrics?.length ? (
						<div className="relative flex-1 self-stretch">
							<ErrorBoundary>
								<ParentSize className="absolute">
									{({ height, width }) => (
										<AreaGraph
											data={epochMetrics}
											height={height}
											width={width}
											getX={({ epoch }) => Number(epoch)}
											getY={({ epochTotalTransactions }) => Number(epochTotalTransactions)}
											color="black"
											formatY={formatAmount}
											tooltipContent={TooltipContent}
										/>
									)}
								</ParentSize>
							</ErrorBoundary>
						</div>
					) : (
						<Text color="steel" variant="body/medium">
							No historical data available
						</Text>
					)}
				</div>
			</div>
		</Card>
	);
}
