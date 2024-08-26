#!/bin/bash
#cargo nextest run --profile ci  --retries 1
#https://nexte.st/book/filter-expressions.html
# --jobs 11 【limit working threads】
#export TMPDIR=/path/to/custom/temp/dir
#nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci --retries 1 > cargo_logs 2>&1 &
cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1

