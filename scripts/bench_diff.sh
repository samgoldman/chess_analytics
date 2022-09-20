#!/bin/bash

git checkout $1
cargo criterion
git checkout $2
cargo criterion --message-format=json | tee bench_results.log

python ./scripts/check_for_regressions.py bench_results.log
