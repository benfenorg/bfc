#!/bin/bash
#cargo nextest run --profile ci  --retries 1
#https://nexte.st/book/filter-expressions.html
# --jobs 11 【limit working threads】
cargo nextest run -E 'not test(/sim_.*/)' --profile ci  --retries 1

