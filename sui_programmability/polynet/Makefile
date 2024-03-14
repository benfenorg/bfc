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
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 183963f59606d3b8fc8182d8f2da80255b267efd9715275f19eaa9e9014f5d8d

