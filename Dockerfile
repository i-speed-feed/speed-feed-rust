FROM rust:latest as builder

WORKDIR /opt/speed-feed

RUN apt-get update \
    && apt-get install -y gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf \
    libssl-dev \
    git

RUN cd /opt \
    && git clone https://github.com/openssl/openssl \
    && cd openssl \
    && git checkout OpenSSL_1_0_2

RUN cd /opt/openssl \
    && mkdir /opt/sysroot \
    && ./Configure os/compiler:arm-linux-gnueabihf-gcc --prefix=/opt/sysroot \
    && make \
    && make install

RUN rustup target install armv7-unknown-linux-gnueabihf

ADD . /opt/speed-feed

RUN export OPENSSL_DIR=/opt/sysroot && cargo build --release --target armv7-unknown-linux-gnueabihf

FROM arm32v7/rust:1.40

COPY --from=builder /opt/speed-feed/target/armv7-unknown-linux-gnueabihf/release/speed_feed /usr/bin/speed_feed

CMD [ "speed_feed" ]