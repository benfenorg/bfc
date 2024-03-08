all: compile publish compile publish test

compile:
	bfc move build --skip-fetch-latest-git-deps

test:
	bfc move test --skip-fetch-latest-git-deps -s --coverage

coverage:
	bfc move coverage summary

publish:
	bfc client publish --gas-budget 2000000000  | tee publish.log

#note: should replace real upgrade_cap_id

upgrade:
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 0x6f7fa31f7d9f5fb81d6365d6c2908288f5c77ca630a9f63933cfb17eaee76d36

