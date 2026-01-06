FROM docker.io/rust:1-slim-bookworm AS build

## cargo package name: customize here or provide via --build-arg
ARG pkg=rust-helloworld

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target                                    \
    --mount=type=cache,target=/usr/local/cargo/registry                        \
    --mount=type=cache,target=/usr/local/cargo/git                             \
    set -eux;                                                                  \
    cargo build --release;                                                     \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################

FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install curl -y

WORKDIR /app

## copy the main binary
COPY --from=build /build/main ./

## copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates

ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./main"]

