#!/bin/sh
# This is a script to test the parsing of many files at once.
# Expects a list of files as stdin.
#
# Usage:
# find <directory> -name '*.js' | ./bulk-check.sh <expected exit code>
#
# Example passing tests:
#  find passing/ -name '*.js' | ./bulk-check.sh 0
#
# Example failing tests:
# find failing/ -name '*.js' | ./bulk-check.sh 1

ROOT="$(dirname $0)"
FAJT="$ROOT/target/debug/fajt"

EXPECTED_EXIT_CODE=$1

if [ -z "$EXPECTED_EXIT_CODE" ]; then
  echo "Usage: $0 <expected exit code>"
  exit 1
fi

if [ ! -f "$FAJT" ]; then
  (cd $ROOT && cargo build)
fi

FAILED=0
PASSED=0
while read FILE
do
  $FAJT "$FILE" 1>/dev/null 2>&1

  if [ $? -ne $EXPECTED_EXIT_CODE ]; then
    FAILED=$((FAILED + 1))
    echo "Failed: $FILE"
  else
    PASSED=$((PASSED + 1))
  fi
done

>&2 echo "Passed: $PASSED"
>&2 echo "Failed: $FAILED"
>&2 echo "$((PASSED * 100 / (PASSED + FAILED)))% success"
