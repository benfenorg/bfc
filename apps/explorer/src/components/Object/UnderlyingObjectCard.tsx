// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useGetDynamicFieldObject, useGetNormalizedMoveStruct } from '@mysten/core';
import { getObjectFields, getObjectType } from '@mysten/sui.js';
=======
import { useDynamicFieldObject, useNormalizedMoveStruct } from '@mysten/dapp-kit';
>>>>>>> heads/mainnet-v1.9.1
import { LoadingIndicator } from '@mysten/ui';

import { FieldItem } from './FieldItem';
import { Banner } from '~/ui/Banner';
<<<<<<< HEAD

interface UnderlyingObjectCardProps {
	parentId: string;
	name: {
		type: string;
		value?: string;
	};
=======

import type { DynamicFieldName } from '@mysten/sui.js/client';

interface UnderlyingObjectCardProps {
	parentId: string;
	name: DynamicFieldName;
>>>>>>> heads/mainnet-v1.9.1
	dynamicFieldType: 'DynamicField' | 'DynamicObject';
}

export function UnderlyingObjectCard({
	parentId,
	name,
	dynamicFieldType,
}: UnderlyingObjectCardProps) {
<<<<<<< HEAD
	const { data, isLoading, isError, isFetched } = useGetDynamicFieldObject(parentId, name);
	const objectType = data ? getObjectType(data!) : null;
=======
	const { data, isLoading, isError, isFetched } = useDynamicFieldObject({ parentId, name });
	const objectType =
		data?.data?.type ??
		(data?.data?.content?.dataType === 'package' ? 'package' : data?.data?.content?.type) ??
		null;
>>>>>>> heads/mainnet-v1.9.1
	// Get the packageId, moduleName, functionName from the objectType
	const [packageId, moduleName, functionName] = objectType?.split('<')[0]?.split('::') || [];

	// Get the normalized struct for the object
	const {
		data: normalizedStruct,
		isFetched: normalizedStructFetched,
		isLoading: loadingNormalizedStruct,
<<<<<<< HEAD
	} = useGetNormalizedMoveStruct({
		packageId,
=======
	} = useNormalizedMoveStruct({
		package: packageId,
>>>>>>> heads/mainnet-v1.9.1
		module: moduleName,
		struct: functionName,
	});

	// Check for error first before showing the loading spinner to avoid infinite loading if GetDynamicFieldObject fails
	if (
		isError ||
		(data && data.error) ||
		(isFetched && !data) ||
		(!normalizedStruct && normalizedStructFetched)
	) {
		return (
			<Banner variant="error" spacing="lg" fullWidth>
				Failed to get field data for {parentId}
			</Banner>
		);
	}

	if (isLoading || loadingNormalizedStruct) {
		return (
			<div className="mt-3 flex w-full justify-center pt-3">
				<LoadingIndicator text="Loading data" />
			</div>
		);
	}

<<<<<<< HEAD
	const fieldsData = getObjectFields(data);
=======
	const fieldsData =
		data.data?.content?.dataType === 'moveObject' ? data.data?.content.fields : null;
>>>>>>> heads/mainnet-v1.9.1
	// Return null if there are no fields
	if (!fieldsData || !normalizedStruct?.fields || !objectType) {
		return null;
	}
	// For dynamicObject type show the entire object
<<<<<<< HEAD
	const fieldData = dynamicFieldType === 'DynamicObject' ? fieldsData : fieldsData?.value;
=======
	const fieldData =
		dynamicFieldType === 'DynamicObject' ? fieldsData : (fieldsData as { value?: unknown })?.value;
>>>>>>> heads/mainnet-v1.9.1

	const dynamicFieldsData =
		// show name if it is a struct
		typeof name.value === 'object' ? { name, value: fieldData } : fieldData;

	return (
		<FieldItem
<<<<<<< HEAD
			value={dynamicFieldsData}
=======
			value={dynamicFieldsData as string}
>>>>>>> heads/mainnet-v1.9.1
			objectType={objectType}
			// add the struct type to the value
			type={normalizedStruct?.fields.find((field) => field.name === 'value')?.type || ''}
		/>
	);
}
