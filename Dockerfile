ARG PROFILE="release"

FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG PROFILE
RUN apt update && apt install -y openssl libssl-dev pkg-config
COPY --from=planner /app/recipe.json recipe.json
# cargo chef cook
RUN cargo chef cook \
  $(if [ "$PROFILE" = "release" ]; then echo --release; fi) \
  --recipe-path recipe.json
COPY . .
# cargo build
RUN cargo build \
  $(if [ "$PROFILE" = "release" ]; then echo --release; fi) \
  && mv ./target/$(if [ "$PROFILE" = "release" ]; then echo release; else echo debug; fi)/hatsu ./target/hatsu

FROM bitnami/minideb:bookworm AS runtime
WORKDIR /app
RUN apt update && apt install -y openssl libssl-dev pkg-config ca-certificates
COPY --from=builder /app/target/hatsu /app/
EXPOSE 3939/tcp
CMD [ "/app/hatsu" ]
