package main

import (
	"bufio"
	"fmt"
	"io"
	"net"
	"time"
)

func Client(addr string) error {
	tcpAddr, err := net.ResolveTCPAddr("tcp", addr)
	if err != nil {
		panic(err)
	}

	conn, err := net.DialTCP("tcp4", nil, tcpAddr)
	if err != nil {
		panic(err)
	}

	go func() {
		ticker := time.NewTicker(1 * time.Second)
		for {
			<-ticker.C
			conn.Write([]byte(time.Now().String() + "\n"))
		}
	}()

	// but kodak, why didn't you use io.Copy?
	go func() {
		for {
			reader := bufio.NewReader(conn)
			bytes, err := reader.ReadBytes(byte('\n'))
			if err != nil {
				if err != io.EOF {
					panic(err)
				}
				return
			}
			fmt.Printf("server [%s] sent: %s", conn.RemoteAddr(), bytes)
		}
	}()

	return nil

}
