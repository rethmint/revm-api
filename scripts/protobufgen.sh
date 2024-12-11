#!/usr/bin/env bash
set -e
echo "Generating gogo proto code"

cd proto
proto_dirs=$(find ./ev, -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq)
for dir in $proto_dirs; do
  for file in $(find "${dir}" -maxdepth 1 -name '*.proto'); do
    protoc -I . -I "$GOPATH/src" --gogo_out=plugins=grpc:. "${file}"
  done
done
cd ..

