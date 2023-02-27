FROM rust:1-bullseye AS builder

WORKDIR /usr/src/catapult
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim

WORKDIR /catapult
COPY --from=builder /usr/local/cargo/bin/catapult /usr/local/bin/catapult
COPY Rocket.toml ./

EXPOSE 8000/tcp
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_LOG_LEVEL=normal
CMD ["catapult"]
