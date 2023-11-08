#!/bin/bash

GO_OUT_DIR=./client

protoc \
    --experimental_allow_proto3_optional \
    --go_out=$GO_OUT_DIR \
    --go-grpc_out=$GO_OUT_DIR \
    ./proto/forester.proto

