FROM ubuntu:22.04
COPY ./target/release/cloudflare-bridge ./target/release/cloudflare-bridge
ENTRYPOINT ["./target/release/cloudflare-bridge"]