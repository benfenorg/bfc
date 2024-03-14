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
	bfc client upgrade --gas-budget 2000000000 --upgrade-capability 4fa0c313d946271e84647a955e957302ad42add59de760afc9afe61f95b2835d

