module github.com/as207960/epp-proxy/v2

go 1.15

require (
	github.com/as207960/epp-proxy/gen/go/epp v0.0.0
	github.com/golang/glog v1.2.2
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.21.0
	google.golang.org/grpc v1.65.0
	google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.5.1
	google.golang.org/protobuf v1.34.2
)

replace github.com/as207960/epp-proxy/gen/go/epp => ./gen/go
