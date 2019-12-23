FROM rust:latest as builder

WORKDIR /opt/speed-feed

RUN apt-get update \
    && apt-get install -y gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf

RUN rustup target install ${arch} 

ADD . /opt/speed-feed

RUN cargo build --release --target armv7-unknown-linux-gnueabihf

RUN ls -la /opt/speed-feed/target/armv7-unknown-linux-gnueabihf/release

FROM arm32v7/rust:1.40

COPY --from=builder /opt/speed-feed/target/armv7-unknown-linux-gnueabihf/release/speed_feed /usr/bin/speed_feed

CMD [ "speed_feed" ]