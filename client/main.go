package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"time"

	pb "chop-client/pb"
	"chop-client/prompts"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	defaultName = "forester"
)

var (
	addr = flag.String("addr", "localhost:50051", "the address to connect to")
	name = flag.String("name", defaultName, "Name I guess")
)

func get(client pb.ForesterClient, key string) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	r, err := client.Get(ctx, &pb.ForesterGetRequest{Key: key})

	if err != nil {
		fmt.Println("could not get key", key, ". Error:", err)
	}

	if r.Value != nil {
		fmt.Println(*r.Value)
	} else {
		fmt.Println("(nil)")
	}
}

func set(client pb.ForesterClient, key string, value string) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	_, err := client.Set(ctx, &pb.ForesterSetRequest{Key: key, Value: value})

	if err != nil {
		fmt.Println("could not set key", key, ". Error:", err)
	}

	fmt.Println("OK")
}

func del(client pb.ForesterClient, key string) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	_, err := client.Delete(ctx, &pb.ForesterDeleteRequest{Key: key})

	if err != nil {
		fmt.Println("could not delete key", key, ". Error:", err)
	}

	fmt.Println("OK")
}

func main() {
	flag.Parse()
	conn, err := grpc.Dial(*addr, grpc.WithTransportCredentials(insecure.NewCredentials()))

	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	c := pb.NewForesterClient(conn)

	for {
		data, err := prompts.PromptForCommand()

		if err != nil {
			fmt.Fprintln(os.Stderr, "Error:", err)
			continue
		}

		switch data.Code {
		case prompts.GET:
			command, err := prompts.ParseGetCommand(data.Values)

			if err != nil {
				fmt.Fprintln(os.Stderr, "Error:", err)
				continue
			}

			get(c, command.Key)

		case prompts.SET:
			command, err := prompts.ParseSetCommand(data.Values)

			if err != nil {
				fmt.Fprintln(os.Stderr, "Error:", err)
				continue
			}

			set(c, command.Key, command.Value)

		case prompts.DELETE:
			command, err := prompts.ParseDeleteCommand(data.Values)

			if err != nil {
				fmt.Fprintln(os.Stderr, "Error:", err)
				continue
			}

			del(c, command.Key)

		case prompts.EXIT:
			fmt.Println("Bye~")
			os.Exit(0)
		}
	}
}
