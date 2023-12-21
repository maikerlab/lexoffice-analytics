#!/bin/bash
OPENAPI_SPEC=$1
GENERATOR=$2
OUT_DIR=$3

if [[ -z $OPENAPI_SPEC ]]; then
  echo "1st parameter must be the openapi spec"
  exit
fi

echo "Generate API Client from spec $OPENAPI_SPEC for $GENERATOR in folder $OUT_DIR"

openapi-generator-cli generate -i $OPENAPI_SPEC -g $GENERATOR -o $OUT_DIR
