#!/usr/bin/env bash

grep -A1 -r '\#\[test\]' src/lib.rs | grep -oP '(?<=fn).*?(?=\()' > lib-tests.txt
grep -A1 -r '\#\[test\]' src/fib.rs | grep -oP '(?<=fn).*?(?=\()' > fib-tests.txt
cat lib-tests.txt
cat fib-tests.txt
circleci tests glob "*.txt" | circleci tests split > tests-to-run
cat tests-to-run

while read name;
do
    cargo test "$name" --release
done <$(cat tests-to-run)

echo "Tests complete"
