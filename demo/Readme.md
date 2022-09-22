# echo client & server
A simple tcp echo client and server to verify operates over the subway tunnel

# How to run
- `go build .` to build the demo
- `./demo` to run the client
- `./demo server` to run the server

# Verification
If everything is working correctly, the client will attempt to connect to the tcp server, but the tcp server will see the ip address of the exit node as the client ip.