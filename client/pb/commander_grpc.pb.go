// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v3.6.1
// source: commander.proto

package pb

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	Commander_Get_FullMethodName    = "/commander.Commander/Get"
	Commander_Set_FullMethodName    = "/commander.Commander/Set"
	Commander_Delete_FullMethodName = "/commander.Commander/Delete"
)

// CommanderClient is the client API for Commander service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type CommanderClient interface {
	Get(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error)
	Set(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error)
	Delete(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error)
}

type commanderClient struct {
	cc grpc.ClientConnInterface
}

func NewCommanderClient(cc grpc.ClientConnInterface) CommanderClient {
	return &commanderClient{cc}
}

func (c *commanderClient) Get(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error) {
	out := new(CommanderResponse)
	err := c.cc.Invoke(ctx, Commander_Get_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *commanderClient) Set(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error) {
	out := new(CommanderResponse)
	err := c.cc.Invoke(ctx, Commander_Set_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *commanderClient) Delete(ctx context.Context, in *CommanderRequest, opts ...grpc.CallOption) (*CommanderResponse, error) {
	out := new(CommanderResponse)
	err := c.cc.Invoke(ctx, Commander_Delete_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// CommanderServer is the server API for Commander service.
// All implementations must embed UnimplementedCommanderServer
// for forward compatibility
type CommanderServer interface {
	Get(context.Context, *CommanderRequest) (*CommanderResponse, error)
	Set(context.Context, *CommanderRequest) (*CommanderResponse, error)
	Delete(context.Context, *CommanderRequest) (*CommanderResponse, error)
	mustEmbedUnimplementedCommanderServer()
}

// UnimplementedCommanderServer must be embedded to have forward compatible implementations.
type UnimplementedCommanderServer struct {
}

func (UnimplementedCommanderServer) Get(context.Context, *CommanderRequest) (*CommanderResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Get not implemented")
}
func (UnimplementedCommanderServer) Set(context.Context, *CommanderRequest) (*CommanderResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Set not implemented")
}
func (UnimplementedCommanderServer) Delete(context.Context, *CommanderRequest) (*CommanderResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Delete not implemented")
}
func (UnimplementedCommanderServer) mustEmbedUnimplementedCommanderServer() {}

// UnsafeCommanderServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to CommanderServer will
// result in compilation errors.
type UnsafeCommanderServer interface {
	mustEmbedUnimplementedCommanderServer()
}

func RegisterCommanderServer(s grpc.ServiceRegistrar, srv CommanderServer) {
	s.RegisterService(&Commander_ServiceDesc, srv)
}

func _Commander_Get_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CommanderRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(CommanderServer).Get(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Commander_Get_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(CommanderServer).Get(ctx, req.(*CommanderRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Commander_Set_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CommanderRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(CommanderServer).Set(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Commander_Set_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(CommanderServer).Set(ctx, req.(*CommanderRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Commander_Delete_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CommanderRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(CommanderServer).Delete(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Commander_Delete_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(CommanderServer).Delete(ctx, req.(*CommanderRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Commander_ServiceDesc is the grpc.ServiceDesc for Commander service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Commander_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "commander.Commander",
	HandlerType: (*CommanderServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Get",
			Handler:    _Commander_Get_Handler,
		},
		{
			MethodName: "Set",
			Handler:    _Commander_Set_Handler,
		},
		{
			MethodName: "Delete",
			Handler:    _Commander_Delete_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "commander.proto",
}
