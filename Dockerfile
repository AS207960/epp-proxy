FROM ekidd/rust-musl-builder as builder

RUN USER=rust cargo init
RUN mkdir static
COPY --chown=rust:rust Cargo.toml Cargo.lock ./
RUN USER=rust cargo build --release

ADD --chown=rust:rust . ./
RUN USER=rust cargo build --release

FROM scratch

COPY --from=builder --chown=0:0 /etc/ssl/certs /etc/ssl/certs
COPY --from=builder --chown=0:0 /home/rust/src/target/x86_64-unknown-linux-musl/release/epp-proxy /

ENTRYPOINT ["/epp-proxy"]
