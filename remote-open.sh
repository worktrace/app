#!/bin/bash

dim() { echo "\033[2m$1\033[0m"; }
red() { echo "\033[31m$1\033[0m"; }
green() { echo "\033[32m$1\033[0m"; }
yellow() { echo "\033[33m$1\033[0m"; }
blue() { echo "\033[34m$1\033[0m"; }

open_in_browser() {
  os=$(uname)
  if [ $os = "Linux" ]; then
    xdg-open $1
  elif [ $os = "Darwin" ]; then # MacOS.
    open $1
  elif [ ${os#$"MINGW64_NT"} != ${os} ]; then # MinGW on Windows.
    start $1
  else
    echo "$(yellow "unsupported os: $os")"
    exit 1
  fi
}

remotes=$(git remote)
if [ -z "$remotes" ]; then
  echo "$(yellow "no branches found.")"
  exit 1
fi

for remote in $remotes; do
  raw=$(git remote get-url "$remote")
  raw=${raw#$"https://"} # Case https mode.
  raw=${raw#$"git@"}     # Case ssh mode.
  raw=${raw#$"code."}    # Especially for GitLink.
  raw=${raw%".git"}
  raw=${raw/":"/"/"}
  open_in_browser "https://$raw"
done
