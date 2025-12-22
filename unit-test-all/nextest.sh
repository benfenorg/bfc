#!/bin/bash
#UPDATE_BASELINE=1 cargo nextest run --profile ci  --retries 1
#https://nexte.st/book/filter-expressions.html
# --jobs 11 【limit working threads】
#export TMPDIR=/path/to/custom/temp/dir
#nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci --retries 1 --jobs 16 > cargo_logs 2>&1 &
#nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci --retries 1 --jobs 16 > cargo_logs 2>&1 &
#nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci --retries 1 --jobs 16 > cargo_logs 2>&1 &

cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1 --jobs 16


UPDATE_BASELINE=1 cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1 --jobs 16

#ensure log output sample
 cargo test --package sui-cluster-test --test local_cluster_test test_sui_cluster  -- --nocapture