package main

import (
  "context"
  "flag"
  "net/http"

  "github.com/golang/glog"
  "github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
  "google.golang.org/grpc"
  "google.golang.org/grpc/credentials"

  gw "github.com/as207960/epp-proxy/gen/go/epp"
)

var (
  listen = flag.String("listen", ":8081", "Address to listen for HTTP requests on")
  grpcServerEndpoint = flag.String("grpc-server-endpoint", "localhost:9090", "gRPC server endpoint")
  grpcServerCert = flag.String("grpc-server-cert", "./root.pem", "gRPC server CA certificate")
)

func run() error {
  ctx := context.Background()
  ctx, cancel := context.WithCancel(ctx)
  defer cancel()

  creds, err := credentials.NewClientTLSFromFile(*grpcServerCert, "");
  if err != nil {
    return err
  }
  mux := runtime.NewServeMux()
  opts := []grpc.DialOption{grpc.WithTransportCredentials(creds)}
  err = gw.RegisterEPPProxyHandlerFromEndpoint(ctx, mux,  *grpcServerEndpoint, opts)
  if err != nil {
    return err
  }

  // Start HTTP server (and proxy calls to gRPC server endpoint)
  return http.ListenAndServe(*listen, mux)
}

func main() {
  flag.Parse()
  defer glog.Flush()

  if err := run(); err != nil {
    glog.Fatal(err)
  }
}
