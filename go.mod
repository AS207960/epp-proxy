module github.com/as207960/epp-proxy/v2

go 1.24.0

require (
	github.com/as207960/epp-proxy/gen/go/epp v0.0.0
	github.com/golang/glog v1.2.5
	github.com/grpc-ecosystem/grpc-gateway/v2 v2.27.1
	google.golang.org/grpc v1.79.3
	google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.5.1
	google.golang.org/protobuf v1.36.10
)

require (
	github.com/kr/text v0.2.0 // indirect
	github.com/rogpeppe/go-internal v1.14.1 // indirect
	golang.org/x/net v0.48.0 // indirect
	golang.org/x/sys v0.39.0 // indirect
	golang.org/x/text v0.32.0 // indirect
	google.golang.org/genproto/googleapis/api v0.0.0-20251202230838-ff82c1b0f217 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20251202230838-ff82c1b0f217 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

replace github.com/as207960/epp-proxy/gen/go/epp => ./gen/go
