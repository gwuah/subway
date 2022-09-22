package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
)

func Server(addr string) error {
	tcpAddr, err := net.ResolveTCPAddr("tcp", addr)
	if err != nil {
		panic(err)
	}

	listener, err := net.ListenTCP("tcp4", tcpAddr)
	if err != nil {
		panic(err)
	}

	for {

		conn, err := listener.AcceptTCP()
		if err != nil {
			log.Println("failed to accept tcp connection: ", err)
			continue
		}

		go func(conn *net.TCPConn) {
			defer conn.Close()

			// but kodak why didn't you use io.Copy
			reader := bufio.NewReader(conn)
			for {
				bytes, err := reader.ReadBytes(byte('\n'))
				if err != nil {
					panic(err)
				}
				fmt.Printf("client [%s] sent: %s", conn.RemoteAddr(), bytes)
				conn.Write([]byte(bytes))
			}
		}(conn)

	}

}
