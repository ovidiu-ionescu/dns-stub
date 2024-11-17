#!/bin/bash
# Creates a dummy network interface for local testing

SUDO=''
if (( EUID != 0 )); then
    SUDO='sudo'
fi

LINK_NAME='dns-stub'
IP_ADDRESS='192.168.0.10'

$SUDO ip link add ${LINK_NAME} type dummy
$SUDO ip addr add ${IP_ADDRESS}/24 dev ${LINK_NAME}
$SUDO ip link set ${LINK_NAME} up

# remove using
# ip link del dns-stub
#
# tell splitdns to use the address itself for the dns server

$SUDO resolvectl dns ${LINK_NAME} ${IP_ADDRESS}

