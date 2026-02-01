#!/bin/bash
set -e
source $HOME/.cargo/env
cd /home/ubuntu/projects/clawpad
cargo check
echo "Build verification successful."
