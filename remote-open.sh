#!/bin/bash

# open_in_browser() {
#   url="$1"
#   case "$(uname -s)" in
#   Darwin)
#     open "$url"
#     ;;
#   MINGW* | CYGWIN*)
#     start "$url"
#     ;;
#   *)
#     xdg-open "$url"
#     ;;
#   esac
# }

# # 获取所有的远程地址
# remotes=$(git remote -v | awk '$3 == "push" {print $2}')

# # 遍历每个远程地址
# for remote in $remotes; do
#   # 转换 SSH 地址为 HTTPS 地址
#   if echo "$remote" | grep -q "^git@"; then
#     host=$(echo "$remote" | cut -d ':' -f 1 | cut -d '@' -f 2)
#     repo=$(echo "$remote" | cut -d ':' -f 2)
#     # 去除 .git 尾缀
#     repo=${repo%.git}
#     https_url="https://$host/$repo"
#   else
#     https_url=$remote
#     https_url=${https_url%.git}
#   fi

#   # 调用函数通过默认浏览器打开 URL
#   open_in_browser "$https_url"
# done

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
