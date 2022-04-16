#!/bin/bash

protoc -I proto/ --grpc-gateway_out ./gen/go --grpc-gateway_opt logtostderr=true --grpc-gateway_opt paths=source_relative --grpc-gateway_opt generate_unbound_methods=true proto/epp.proto
protoc -I proto/ --openapiv2_out ./gen/openapiv2 --openapiv2_opt logtostderr=true --openapiv2_opt generate_unbound_methods=true proto/epp.proto
