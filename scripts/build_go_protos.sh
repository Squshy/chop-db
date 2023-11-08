#!/bin/bash

GO_OUT_DIR=./client

protoc --go_out=$GO_OUT_DIR --go-grpc_out=$GO_OUT_DIR ./proto/lumberjack.proto

