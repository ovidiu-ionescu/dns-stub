#!/bin/bash
#
set -e
cargo build
sudo setcap 'cap_net_bind_service=+ep' target/debug/dns-stub

IP=$(ip addr show dev dns-stub | grep 'inet ' | awk '{print $2}' | cut -d'/' -f1)
RESPONSE=$(minikube ip)
RUST_LOG=debug cargo run -- -i $IP -p 53 -r $RESPONSE -d



