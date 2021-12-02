#!/bin/sh

hook_dest=$(git rev-parse --show-toplevel)/.git/hooks
hook_source=$(dirname $(realpath $0))

set -e

cp -v $hook_source/pre-commit $hook_dest/pre-commit
