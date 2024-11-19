#!/bin/bash
# Configure split DNS to use the dns-stub for simulacron.eu and the regular 
# network adapter for all other domains.
#
NT=$(ip route get 9.9.9.9 | grep dev | cut -d ' ' -f5)
echo "Using network adapter for general DNS: $NT"

sudo resolvectl dns dns-stub 192.168.0.10

sudo resolvectl domain dns-stub lab ${1:-simulacron.eu}
sudo resolvectl domain $NT '~.'
