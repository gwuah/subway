# Create all namespaces
sudo ip netns add boxA
sudo ip netns add router
sudo ip netns add boxB

# Create veth pairs and move them into their respective namespaces
sudo ip link add veth0 type veth peer name veth1
sudo ip link set veth0 netns boxA
sudo ip link set veth1 netns router
sudo ip link add veth2 type veth peer name veth3
sudo ip link set veth3 netns boxB
sudo ip link set veth2 netns router

# Assign IP addresses 
sudo ip netns exec boxA ip addr add 172.16.100.5/24 dev veth0
sudo ip netns exec router ip addr add 172.16.100.1/24 dev veth1
sudo ip netns exec boxB ip addr add 172.16.200.5/24 dev veth3
sudo ip netns exec router ip addr add 172.16.200.1/24 dev veth2

# Bring up devices
sudo ip netns exec boxA ip link set dev veth0 up
sudo ip netns exec router ip link set dev veth1 up
sudo ip netns exec router ip link set dev veth2 up
sudo ip netns exec boxB ip link set dev veth3 up

# Enable forwarding globally
echo 1 >  /proc/sys/net/ipv4/ip_forward

# Enable logging from within a namespace
echo  1 > /proc/sys/net/netfilter/nf_log_all_netns