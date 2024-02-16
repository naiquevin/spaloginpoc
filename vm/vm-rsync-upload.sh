#!/usr/bin/env bash

set -e

src=$1
target=$2
# Rest of the args to be sent to rsync
shift 2
rsync_args="$@"

if [ -z "$src" ]; then
    echo First arg required
    exit 1
fi

if [ -z "$target" ]; then
    echo "Considering ~/ as the default target"
    target="~/"
fi

rsync -e "ssh -i ~/.ssh/mpvm_user_key" \
      $rsync_args \
      $src \
      vmadmin@$(multipass info nginx-spa | grep -i ipv4 | awk '{ print $2 }'):$target
