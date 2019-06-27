#!/bin/bash
# Script to build the symrs project

set -e

export RUSTDOCFLAGS="--html-in-header build/katex.html"
export TMPL_BLOCK_END_STR='%}'
export TMPL_BLOCK_START_STR='// {%'
export TMPL_COMMENT_END_STR='#}'
export TMPL_COMMENT_START_STR='// {#'
export TMPL_LOG_FILE="tmpl.log"
export TMPL_LOG_LEVEL="debug"
export TMPL_LOG_MODE="overwrite"
export TMPL_VAR_END_STR='}}'
export TMPL_VAR_START_STR='{{'

tmpl_cmd="tmpl src --delete --output build"

if [ "$#" -eq 0 ]; then
    cargo
elif [ "$#" -eq 1 ]; then
    if [ "$1" == "clean" ]; then
        rm -rf build
    else
        $tmpl_cmd && cargo $@
    fi
else
    $tmpl_cmd && cargo $@
fi
