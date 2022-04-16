FROM ekidd/rust-musl-builder:nightly-2021-12-23 as builder

RUN USER=rust cargo init
RUN mkdir static

ADD --chown=rust:rust . ./
RUN USER=rust cargo build --release

FROM scratch

COPY --from=builder --chown=0:0 /etc/ssl/certs /etc/ssl/certs
COPY --from=builder --chown=0:0 /home/rust/src/target/x86_64-unknown-linux-musl/release/epp-proxy /

ENTRYPOINT ["/epp-proxy"]
