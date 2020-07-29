FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/catapult
COPY . .
RUN cargo install --path .


FROM debian:buster-slim

WORKDIR /catapult
COPY --from=builder /usr/local/cargo/bin/catapult /usr/local/bin/catapult
COPY Rocket.toml ./

EXPOSE 8000/tcp
CMD ["catapult"]
