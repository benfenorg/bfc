#!/bin/bash
cargo nextest run 'not(bfc_sim(/))' --profile ci  --retries 1

