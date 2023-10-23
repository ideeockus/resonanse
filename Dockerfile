FROM debian:12-slim as builder
LABEL authors="radmirkus"

RUN apt update && apt upgrade -y
RUN apt install -y curl build-essential libclang-dev pkg-config

# install rust & rust components
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"
#RUN rustup target add arm-unknown-linux-gnueabihf
#RUN rustup toolchain install stable-arm-unknown-linux-gnueabihf
#RUN rustup component add clippy
#RUN rustup component add rustfmt


WORKDIR /app
COPY . /app

#ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc CC_arm_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc CXX_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
#ENV C_INCLUDE_PATH=/usr/arm-linux-gnueabihf/include/


#ENV CLIPPY_CMD="cargo clippy --release --target arm-unknown-linux-gnueabihf -- --D warnings"
#ENV RUSTFMT_CMD="cargo fmt --target arm-unknown-linux-gnueabihf --check"
#ENV TESTS_CMD="cargo test"
#ENV BUILD_CMD="cargo build --release --target arm-unknown-linux-gnueabihf"
#CMD $RUSTFMT_CMD ; $TESTS_CMD ; $CLIPPY_CMD ; $BUILD_CMD
RUN cargo update

#RUN cargo clippy --release --workspace -- --D warnings
#RUN cargo fmt --check
#RUN cargo test
RUN cargo build --release

FROM debian:12-slim


RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC

WORKDIR /app
COPY --from=builder /resonanse/target/release/resonanse_bot .
CMD ["./resonanse_bot"]

#FROM alpine:latest
