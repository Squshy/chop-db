package main

import (
	"context"
	"flag"
	"log"
	"time"

	pb "chop-client/pb"
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

func get(client pb.ForesterClient, ctx context.Context, key string) {
	r, err := client.Get(ctx, &pb.ForesterGetRequest{Key: key})

	if err != nil {
		log.Fatalf("could not get: %v", err)
	}

	log.Printf("Got: %s", r.Value)
}

func set(client pb.ForesterClient, ctx context.Context, key string, value string) {
	_, err := client.Set(ctx, &pb.ForesterSetRequest{Key: key, Value: value})

	if err != nil {
		log.Fatalf("could not set: %v", err)
	}

	log.Printf("Set: %s:%s", key, value)
}

func del(client pb.ForesterClient, ctx context.Context, key string) {
	deleted, err := client.Delete(ctx, &pb.ForesterDeleteRequest{Key: key})

	if err != nil {
		log.Fatalf("could not delete: %v", err)
	}

	log.Printf("Deleted (%s): %s", deleted, key)
}

func main() {
	flag.Parse()
	conn, err := grpc.Dial(*addr, grpc.WithTransportCredentials(insecure.NewCredentials()))

	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	c := pb.NewForesterClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	get(c, ctx, "hehe")
	set(c, ctx, "hehe", "it is set!")
	get(c, ctx, "hehe")
	del(c, ctx, "hehe")
	get(c, ctx, "hehe")
}
