FROM debian:bookworm as builder
LABEL authors="radmirkus"

RUN apt update
RUN apt install -y curl pkg-config ca-certificates

# install rust & rust components
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . /app

RUN cargo update
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC

WORKDIR /app
COPY --from=builder /app/target/release/resonanse_bot .
CMD ["./resonanse_bot"]
