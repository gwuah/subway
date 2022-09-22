package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
)

// This is shitty code, but that's only because its currently 4:28am in Accra and the guy/girl who invented best practices is asleep.
// So, I can do whatever I want and I decided to do LIL UZI VERT. You got it? uzi vert? do what i want? HAHA im so funny

type config struct {
	port            int
	serverRemoteUrl string
}

func (c config) getLocalServerAddress() string {
	return fmt.Sprintf("0.0.0.0:%v", c.port)
}

func newCancelableContext() context.Context {
	doneCh := make(chan os.Signal, 1)
	signal.Notify(doneCh, os.Interrupt)

	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)

	go func() {
		<-doneCh
		cancel()
	}()

	return ctx
}

func main() {
	ctx := newCancelableContext()
	cfg := config{
		// because why not
		port: 6666,

		// this is a private ip, replace with yours
		serverRemoteUrl: "150.150.150.0",
	}

	if len(os.Args[:]) > 1 {
		go Server(cfg.getLocalServerAddress())
		fmt.Printf("server running @ %s\n", cfg.getLocalServerAddress())
	} else {
		fmt.Printf("client will talk to @ %s\n", cfg.serverRemoteUrl)
		go Client(cfg.serverRemoteUrl)
	}

	<-ctx.Done()
}
