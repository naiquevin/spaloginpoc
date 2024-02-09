#!/usr/bin/env bash

ssh -i ~/.ssh/mpvm_user_key \
    vmadmin@$(multipass info nginx-spa | grep -i ipv4 | awk '{ print $2 }')
