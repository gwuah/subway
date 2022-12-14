# Subway 🚅
Subway is a layer-3(IP) tunneling sofware that routes your internet traffic through an exit-node of your choice. Just like how traditional vpns work, except this time, you own the exit node. You can also add encryption, disable the NAT feature and it will double as a peer-to-peer VPN. I'll recommend using [tailscale](https://github.com/tailscale/tailscale) or [innernet](https://github.com/tonarino/innernet) though.

# Map
- home - contains personal notes & a tcp proxy that communicates over the tunnel.
- node - contains the tunneling software.


# How to use
- Run `make build` in the root repo
- On the machine whose traffic you want to tunnel, run `make run-client` 
- On the exit node, run  `sysctl -w net.ipv4.ip_forward=1` to enable ip forwarding.
- On the exit node, run `make run-server`
- And you should have your own private network pipes on the internet!  🧙‍♀️

# Architecture
Here's the architecture of the client node  
![[client architecture](https://lxd.me/a-simple-vpn-tunnel-with-tun-device-demo-and-some-basic-concepts)](./home/tunc.png)

The server is pretty much a reverse of the client. 
- Traffic arrives at the udp socket.
- We write it to the tun0 virtual interface. 
- The kernel sees our original packet & since ip forwarding is turned on, it forwards the packet to the next hop router/gateway.
- Right before the packets exits the kernel, we perform network address translation on the source IP on the packet so the internet can route a reply back home.


# Notes
- It only works on linux - mainly because the tuntap library I'm using only works on linux.
- It's not very secure - i wrote this software to solidify some networking knowledge, it's not production software.
- It's not very efficient - i wrote this software to solidify some networking knowledge, it's not production software.