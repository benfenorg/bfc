// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ReactComponent as DarkHeaderLogo } from '../../assets/dark_header_logo.svg';
import { ReactComponent as LigthHeaderLogo } from '../../assets/header_logo.svg';

export function HeaderLogo({ isDarker }: any) {
	return isDarker ? <DarkHeaderLogo /> : <LigthHeaderLogo />;
}
