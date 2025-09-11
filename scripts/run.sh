#!/bin/bash

if ! ./scripts/check.sh; then
    exit 1
else
    echo ""
    cargo r
fi
