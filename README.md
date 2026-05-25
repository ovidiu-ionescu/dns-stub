# dns-stub

This a DNS server for local development.

Using a command that no longer has an official support, you can instruct the 
server at runtime to resolve a specific domain name to a specific IP address.

This way, it can be used to assign domain names to docker containers or other
services running on some dummy network interface.

Currently, it only supports A queries and has a default IP address it returns
when the domain has not been assigned a specific address.

It is meant to be used for testing with split DNS resolving, like for instance
`systemd-resolved`

A typical use would be to create a dummy network interface, assign it an non 
routable IP address and use resolvectl to instructs split DNS that on that 
address there is a DNS server listening.\
This functionality is implemented in 
[setup_dummy_interface.sh](scripts/setup_dummy_interface.sh).

Further on, you can assign some domains for split DNS to resolve through this
server:
```bash
sudo resolvectl domain dns-stub test.mydomain.com
```
A helper script is also provided:
```bash
scripts/resolve_dns.sh
```

After that, start the server passing in the command line the IP address of the 
dummy interface and the address it should return in the DNS answers.

The [run-with-minikube.sh](run-with-minikube.sh) script instructs it to return the address of a running 
minikube instance.

If the logging is set to level debug `RUST_LOG=debug` the program will save in 
files all the requests and responses.

## Docker containers
Normally Docker containers are assigned a dynamic address when they are started. Using this server
you could add their IP address in the DNS server and resolve them via regular names.

## Autostart with systemd
Follow instructions here: [autostart](AUTOSTART.md)
