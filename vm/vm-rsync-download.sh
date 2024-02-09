#!/usr/bin/env bash

set -e

src=$1
target=$2

if [ -z "$src" ]; then
    echo 'First arg (src path) required'
    exit 1
fi

if [ -z "$target" ]; then
    echo "Considering pwd as the default target"
    target="."
fi

rsync -e "ssh -i ~/.ssh/mpvm_user_key" \
      vmadmin@$(multipass info nginx-spa | grep -i ipv4 | awk '{ print $2 }'):$src \
      $target
