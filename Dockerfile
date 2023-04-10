FROM rustlang/rust:nightly AS builder
RUN update-ca-certificates
WORKDIR /usr/src/

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip && \
  mkdir -p /root/.local && \
  unzip protoc-3.15.8-linux-x86_64.zip -d /root/.local && \
  rm protoc-3.15.8-linux-x86_64.zip && \
  chmod +x /root/.local/bin/protoc

ENV PROTOC=/root/.local/bin/protoc

RUN cargo init epp-proxy
WORKDIR /usr/src/epp-proxy
RUN mkdir static

ADD . ./
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates p11-kit-modules \
    libengine-pkcs11-openssl && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

COPY --from=builder --chown=0:0 /usr/local/cargo/bin/epp-proxy /epp-proxy

ENTRYPOINT ["/epp-proxy"]
