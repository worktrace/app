#!/bin/bash

dim() { echo "\033[2m$1\033[0m"; }
red() { echo "\033[31m$1\033[0m"; }
green() { echo "\033[32m$1\033[0m"; }
yellow() { echo "\033[33m$1\033[0m"; }
blue() { echo "\033[34m$1\033[0m"; }

remotes=$(git remote)
if [ -z "$remotes" ]; then
  echo "$(yellow "no branches found.")"
  exit 1
fi

success=true
for remote in $remotes; do
  # Branches.
  echo "$(blue "$remote")$(dim ": pushing all branches...")"
  git push "$remote" --all
  if [ $? -eq 0 ]; then
    echo "$(green "$remote")$(dim ": branches pushed successfully.")"
  else
    echo "$(red "$remote")$(dim ": branches pushed failed.")"
    success=false
  fi

  # Tags.
  echo "$(blue "$remote")$(dim ": pushing all tags...")"
  git push "$remote" --tags
  if [ $? -eq 0 ]; then
    echo "$(green "$remote")$(dim ": tags pushed successfully.")"
  else
    echo "$(red "$remote")$(dim ": tags pushed failed.")"
    success=false
  fi
done

if [ "$success" = false ]; then
  echo "$(yellow "some remote syncing failed.")"
  exit 1
fi
echo "$(green "all remote synced.")"
