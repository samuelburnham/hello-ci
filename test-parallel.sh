#!/usr/bin/env bash

grep -A1 -r '\#\[test\]' src/lib.rs | grep -oP '(?<=fn).*?(?=\()' > lib-tests.txt
grep -A1 -r '\#\[test\]' src/fib.rs | grep -oP '(?<=fn).*?(?=\()' > fib-tests.txt
circleci tests glob "src/*.txt" | circleci tests split > tests-to-run
cat tests-to-run

while read name;
do
    cargo test "$name" --release
done <tests-to-run

echo "Tests complete"
