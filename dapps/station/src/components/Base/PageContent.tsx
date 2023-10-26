// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import { useAppDispatch } from "~/state/hooks";
import { updatePathTab } from '~/state/user/reducer';

export default function PageContent({id}:{id:string}) {
    const dispatch = useAppDispatch();
    useEffect(()=>{
        dispatch(updatePathTab({path:id}))
    },[id])
	return (
		<Outlet />
	);
}
