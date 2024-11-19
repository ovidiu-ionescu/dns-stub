# Make the dns-stub service persistent

We need to create a persistent dummy interface and a service to run the dns-stub binary.

To create a persistent dummy interface copy 
```bash
sudo cp 10-dns-stub.netdev /etc/systemd/network/
sudo cp 20-dns-stub.network /etc/systemd/network/
```
To create it without rebooting run
```bash
sudo systemctl restart systemd-networkd
```
Enable the systemd-networkd service
```bash
sudo systemctl enable systemd-networkd
```

Prepare the binary for the service
```bash
cargo build --release
sudo mv target/release/dns-stub /usr/local/bin/
sudo setcap 'cap_net_bind_service=+ep' /usr/local/bin/dns-stub
```

To create the service copy
```bash
sudo cp dns-stub.service /etc/systemd/system/
```

To start the service run
```bash
Reload stuff
```bash
sudo systemctl daemon-reload
sudo systemctl start dns-stub.service
sudo systemctl enable dns-stub.service
```

