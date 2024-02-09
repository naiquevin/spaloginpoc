#!/usr/bin/env bash

multipass launch \
          -c 2 \
          -d 5G \
          -m 1G \
          -n nginx-spa \
          --cloud-init cloudinit.yml \
          jammy
