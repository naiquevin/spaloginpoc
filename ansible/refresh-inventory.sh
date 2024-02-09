#!/usr/bin/env bash

set -e

ipaddress=$(multipass info nginx-spa | grep -i ipv4 | awk '{ print $2 }')

cat <<EOF > inventory.ini
[nginx-spa]
$ipaddress ansible_user=vmadmin ansible_ssh_private_key_file=~/.ssh/mpvm_user_key
EOF
