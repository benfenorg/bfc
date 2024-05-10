// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useCallback, useLayoutEffect, useRef, useState } from 'react';

type Status = 'loading' | 'failed' | 'loaded';

interface UseImageProps {
	src?: string;
	moderate?: boolean;
}

export function useImage({ src = '' }: UseImageProps) {
	const [status, setStatus] = useState<Status>('loading');
	const formatted = src?.replace(/^ipfs:\/\//, 'https://ipfs.io/ipfs/');

	const ref = useRef<HTMLImageElement | null>(null);

	const cleanup = () => {
		if (ref.current) {
			ref.current.onload = null;
			ref.current.onerror = null;
			ref.current = null;
		}
	};

	const load = useCallback(() => {
		if (!src) setStatus('failed');
		const img = new Image();
		img.src = formatted;

		img.onload = () => setStatus('loaded');
		img.onerror = () => setStatus('failed');
		ref.current = img;
	}, [src, formatted]);

	useLayoutEffect(() => {
		load();
		return () => cleanup();
	}, [load]);

	return { url: formatted, status, ref };
}

export default useImage;
