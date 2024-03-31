#!/bin/bash
# Configure split DNS to use the dns-stub for simulacron.eu and the regular 
# network adapter for all other domains.
#
NT=$(ip route get 9.9.9.9 | grep dev | cut -d ' ' -f5)

sudo resolvectl domain dns-stub ${1:-simulacron.eu}
sudo resolvectl domain $NT '~.'
