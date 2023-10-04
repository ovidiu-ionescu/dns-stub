#!/bin/bash
# Creates a dummy network interface for local testing

SUDO=''
if (( $EUID != 0 )); then
    SUDO='sudo'
fi

$SUDO ip link add dns-stub type dummy
$SUDO ip addr add 192.168.0.10/24 dev dns-stub
$SUDO ip link set dns-stub up

# remove using
# ip link del dns-stub
