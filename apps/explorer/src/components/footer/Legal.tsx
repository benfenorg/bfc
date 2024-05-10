// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Link } from '~/ui/Link';

export function LegalText() {
	return (
		<div className="flex justify-start text-body/[18px] font-normal text-bfc-text3 md:justify-start">
			&copy;
			{`${new Date().getFullYear()} BenFen Explorer. All
  rights reserved.`}
		</div>
	);
}

export function LegalLinks() {
	return (
		<ul className="flex flex-row gap-2.5">
			{/* <li className="flex items-center justify-center">
				<Link variant="mono">
					<div className="font-sans text-body/[18px] font-normal text-bfc">Whitepaper</div>
				</Link>
			</li>
			<li className="ml-2.5 flex items-center justify-center">
				<Link variant="mono">
					<div className="font-sans text-body/[18px] font-normal text-bfc">Github</div>
				</Link>
			</li> */}
			<li className="grow" />
			<li className="ml-7.5 flex items-center justify-center">
				<Link variant="mono" href="https://discord.gg/7bkGrQcvmm">
					<svg
						width="16"
						height="16"
						viewBox="0 0 16 16"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<rect width="16" height="16" rx="8" fill="#171719" />
						<path
							d="M11.4656 4.66311C10.8282 4.35615 10.1447 4.13001 9.43006 4.00048C9.41705 3.99798 9.40405 4.00423 9.39734 4.01672C9.30944 4.18082 9.21207 4.39489 9.14389 4.56314C8.37526 4.44237 7.61057 4.44237 6.8577 4.56314C6.7895 4.39115 6.6886 4.18082 6.6003 4.01672C6.59359 4.00465 6.58059 3.9984 6.56758 4.00048C5.85334 4.12959 5.16985 4.35574 4.53208 4.66311C4.52656 4.6656 4.52183 4.66977 4.51869 4.67518C3.22225 6.70803 2.8671 8.69092 3.04133 10.6492C3.04211 10.6588 3.04724 10.668 3.05433 10.6738C3.90969 11.3331 4.73824 11.7333 5.55141 11.9986C5.56442 12.0028 5.57821 11.9978 5.58649 11.9865C5.77885 11.7108 5.95032 11.4201 6.09733 11.1144C6.10601 11.0965 6.09773 11.0753 6.07999 11.0682C5.80802 10.9599 5.54904 10.8279 5.29993 10.678C5.28022 10.6659 5.27864 10.6363 5.29677 10.6221C5.34919 10.5809 5.40163 10.538 5.45169 10.4947C5.46074 10.4868 5.47336 10.4851 5.48401 10.4901C7.1206 11.2744 8.89241 11.2744 10.5097 10.4901C10.5203 10.4847 10.533 10.4864 10.5424 10.4943C10.5925 10.5376 10.6449 10.5809 10.6977 10.6221C10.7158 10.6363 10.7147 10.6659 10.695 10.678C10.4458 10.8308 10.1869 10.9599 9.91449 11.0678C9.89676 11.0749 9.88887 11.0965 9.89755 11.1144C10.0477 11.4197 10.2192 11.7104 10.408 11.9861C10.4159 11.9978 10.4301 12.0028 10.4431 11.9986C11.2602 11.7333 12.0887 11.3331 12.9441 10.6738C12.9516 10.668 12.9563 10.6592 12.9571 10.6496C13.1656 8.38562 12.6079 6.419 11.4786 4.6756C11.4758 4.66977 11.4711 4.6656 11.4656 4.66311ZM6.34173 9.45682C5.849 9.45682 5.44301 8.98204 5.44301 8.39896C5.44301 7.81588 5.84113 7.34109 6.34173 7.34109C6.84626 7.34109 7.24832 7.82004 7.24044 8.39896C7.24044 8.98204 6.84232 9.45682 6.34173 9.45682ZM9.66459 9.45682C9.17188 9.45682 8.76588 8.98204 8.76588 8.39896C8.76588 7.81588 9.16399 7.34109 9.66459 7.34109C10.1691 7.34109 10.5712 7.82004 10.5633 8.39896C10.5633 8.98204 10.1691 9.45682 9.66459 9.45682Z"
							fill="white"
						/>
					</svg>
				</Link>
			</li>
			<li className="flex items-center justify-center">
				<Link variant="mono" href="https://twitter.com/BenFen_Official">
					<svg
						width="16"
						height="16"
						viewBox="0 0 16 16"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<rect width="16" height="16" rx="8" fill="#171719" />
						<path
							fillRule="evenodd"
							clipRule="evenodd"
							d="M6.76166 8.43768L3.19005 12.5145H4.77223L7.50606 9.38568L9.915 12.5151L13.0001 12.4983L9.08864 7.3259L12.4266 3.50233L10.8698 3.48492L8.34845 6.35845L6.19123 3.48962L3 3.48615L6.76166 8.43768ZM11.1317 11.576L10.3332 11.5735L4.84786 4.39222H5.70668L11.1317 11.576Z"
							fill="white"
						/>
					</svg>
				</Link>
			</li>

			{/* {legalLinks.map(({ title, href }) => (
				<li className="flex items-center justify-center" key={href}>
					<Link variant="text" href={href}>
						<Text variant="subtitleSmall/medium" color="steel-darker">
							{title}
						</Text>
					</Link>
				</li>
			))}
			{productAnalyticsConfig?.mustProvideCookieConsent && (
				<li className="flex items-center justify-center">
					<Link variant="text" data-cc="c-settings">
						<Text variant="subtitleSmall/medium" color="steel-darker">
							Manage Cookies
						</Text>
					</Link>
				</li>
			)} */}
		</ul>
	);
}
