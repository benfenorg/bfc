#!/bin/bash
#cargo nextest run --profile ci  --retries 1
#https://nexte.st/book/filter-expressions.html
cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1



#
#nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1  > cargo_logs 2>&1 &

nohup cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1  > cargo_logs 2>&1 && \
nohup cargo simtest --profile ci  --retries 1 > simtest_logs 2>&1 &