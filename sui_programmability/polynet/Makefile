all: compile publish compile publish test

compile:
	bfc move build --skip-fetch-latest-git-deps

test:
	bfc move test --skip-fetch-latest-git-deps -s --coverage

coverage:
	bfc move coverage summary

publish:
	bfc client publish --gas-budget 2000000000 --skip-fetch-latest-git-deps | tee publish.log

#note: should replace real upgrade_cap_id

upgrade:
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 0x759993eae43c864d198b82f893209a224b527069af954e57d94c1e25f3902479

