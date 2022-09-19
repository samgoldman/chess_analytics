#!/bin/bash

BASE=$1
HEAD=$2

git checkout $1
cargo criterion
git checkout $2
cargo criterion --message-format=json | tee bench_results.log
git checkout $1

./scripts/check_for_regressions bench_results.log
