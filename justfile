LINK_NAME := 'dns-stub'
IP_ADDRESS := '192.168.0.10'
RESPONSE := '192.168.0.100'
up:
  sudo ip link add {{LINK_NAME}} type dummy
  sudo ip addr add {{IP_ADDRESS}}/24 dev {{LINK_NAME}}
  sudo ip link set {{LINK_NAME}} up

down:
  sudo ip link del {{LINK_NAME}}

run:
  cargo build
  sudo setcap 'cap_net_bind_service=+ep' target/debug/dns-stub
  RUST_LOG=info cargo run -- -i {{IP_ADDRESS}} -p 53 -r {{RESPONSE}} -d

query:
  dig @{{IP_ADDRESS}} -p 53 ceva.simulacron.eu

update:
  dig @{{IP_ADDRESS}} ceva.simulacron.eu:12.13.14.15 -t TYPE23



