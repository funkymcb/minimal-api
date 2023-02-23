FROM rustlang/rust:nightly-slim as build

WORKDIR /usr/src/minimal-api
COPY . .
RUN cargo clean && cargo build --release

FROM debian:bullseye
ARG APP=/usr/src/app
RUN mkdir -p {$APP}

COPY --from=build /usr/src/minimal-api/target/release/minimal-ui-api-ms ${APP}/minimal-api
COPY --from=build /usr/src/minimal-api/Rocket.toml ${APP}/Rocket.toml

WORKDIR ${APP}

CMD ["./minimal-api"]
