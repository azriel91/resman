#! /bin/bash
set -euo pipefail

cargo llvm-cov clean --workspace
mkdir -p ./target/coverage

# See `.config/cargo.toml`
for i in {0..6}
do cargo coverage_$i
done

cargo llvm-cov report --lcov --output-path ./target/coverage/lcov.info
