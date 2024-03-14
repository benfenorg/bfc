all: compile publish compile publish test

compile:
	bfc move build 

test:
	bfc move test --skip-fetch-latest-git-deps -s --coverage

coverage:
	bfc move coverage summary

publish:
	bfc client publish --gas-budget 2000000000  | tee publish.log

#note: should replace real upgrade_cap_id

upgrade:
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 41c996bbdaaf646e2e5d1166df57b15790af981dfa03317e7e4dba95263d5074

