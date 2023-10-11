export function Overview() {
	return (
		<div className="flex justify-around text-center text-xs px-10 py-5 border border-bf-press rounded-md bg-bf-white_4p">
			<div>
				<div className="text-2xl">1,234.5678</div>
				<div className="text-bf-text3 mt-1">已锁定BFC</div>
			</div>
			<div>
				<div className="text-2xl">4,390.23321</div>
				<div className="text-bf-text3 mt-1">已铸造BUSD</div>
			</div>
			<div>
				<div className="text-2xl">1,238,892,990</div>
				<div className="text-bf-text3 mt-1">用户数</div>
			</div>
		</div>
	);
}
