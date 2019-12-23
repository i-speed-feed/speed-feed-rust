FROM rust:latest as builder

ARG arch="armv7-unknown-linux-gnueabihf"
ARG targetArch="arm32v7"

WORKDIR /opt/speed-feed

RUN apt-get update \
    && apt-get install -y gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf

RUN rustup target install ${arch} 

ADD . /opt/speed-feed

RUN ls -la

RUN cargo build --release --target ${arch}

FROM arm32v7/rust:1.40

COPY --from=builder target/${arch}/speed-feed /opt/speed-feed

CMD [ "/opt/speed-feed" ]