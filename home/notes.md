On Exit Node
- Run `sysctl -w net.ipv4.ip_forward=1` to enable ip forwarding.
- Run `iptables -t nat -A POSTROUTING -s 150.150.150.0/24 -j MASQUERADE` to allow the internet find a way back to your exit node.
- Took inspiration from https://tailscale.com/kb/1103/exit-nodes/#step-1-advertise-a-device-as-an-exit-node

Learnings
- Easy to create routing Loops.
- The first 4 bytes of packets passed through a tuns are required.
- To enable logging from networking namespaces, `echo  1 > /proc/sys/net/netfilter/nf_log_all_netns` and  `tail -f /var/log/syslog`.
- To log all martian packets, run `sysctl -w net.ipv4.conf.default.log_martians=1` && `sysctl -w net.ipv4.conf.all.log_martians=1`
- rp_filters drops packets if route to the packet src can't be found on the local machine. 
- To disable it, run `sysctl net.ipv4.conf.all.rp_filter=0`.
- Unless you're testing stuff in sanboxed network namespaces, disabling it is a very stupid thing to do. Read [rfc3704](https://www.rfc-editor.org/rfc/rfc3704)

Security  
- iptables will accept any traffic. you need to picky about what you let in.
- enabling ip forwarding means anybody can transit stuff through your exit node.
- disabling rp_filter makes you susceptible to all sorts of attacks & flooding.
- we're just adding overhead with no encryption, in wireguard, packet contents are encrypted before being pushed into the internet.
  
Some random commands I used that I don't want to forget  
docker build . -t vpnmachine  
docker run --privileged -it vpnmachine  
docker run --privileged -v $(pwd)/demo:/itto -it vpnmachine  
nsenter --net=/var/run/netns/itto  
echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.conf  
echo 'net.ipv6.conf.all.forwarding = 1' | sudo tee -a /etc/sysctl.conf  
sudo sysctl -p /etc/sysctl.conf  
ip addr flush dev eth0  
iptables -L -n -v -t nat --line-numbers  
tail -f /var/log/syslog  
10.0.0.0/8,172.16.0.0/12 and 192.168.0.0/16  
sysctl net.ipv4.conf.all.rp_filter=0  
```
iptables-save | awk '/^[*]/ { print $1 } 
                     /^:[A-Z]+ [^-]/ { print $1 " ACCEPT" ; }
                     /COMMIT/ { print $0; }' | iptables-restore
```  
iptables -P INPUT ACCEPT  
iptables -P FORWARD ACCEPT  
iptables -P OUTPUT ACCEPT  
iptables -t nat -A POSTROUTING -o tun0 -j SNAT --to 172.18.0.20  
