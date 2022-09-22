run-client:
	./node/target/release/client

run-server:
	./node/target/release/server

check:
	cd node && cargo check

build: 
	cd node && cargo build --release

detach-target:
	mv node/target target

attach-target:
	mv target node/target

fly-sync:
	make detach-target
	cd .. && rsync -a subway/ root@ebpf.fly.dev:/root/subway
	make attach-target

do-sync: 
	make detach-target
	cd .. && rsync -a subway/ root@104.248.237.118:/root/subway
	make attach-target