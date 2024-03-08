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
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 2e34b695b8b1313dd2a4421080240ebbfd1d21dfb60dc57cf10b027db23778d8

