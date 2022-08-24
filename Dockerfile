FROM ekidd/rust-musl-builder:nightly-2021-12-23 as builder

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.15.8/protoc-3.15.8-linux-x86_64.zip && \
  mkdir -p ~/.local && \
  unzip protoc-3.15.8-linux-x86_64.zip -d ~/.local && \
  rm protoc-3.15.8-linux-x86_64.zip && \
  chmod +x ~/.local/bin/protoc

USER rust

ENV PROTOC=/home/rust/.local/bin/protoc

RUN cargo init && mkdir static

ADD --chown=rust:rust . ./
RUN USER=rust cargo build --release

FROM scratch

COPY --from=builder --chown=0:0 /etc/ssl/certs /etc/ssl/certs
COPY --from=builder --chown=0:0 /home/rust/src/target/x86_64-unknown-linux-musl/release/epp-proxy /

ENTRYPOINT ["/epp-proxy"]
