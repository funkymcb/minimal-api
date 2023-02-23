FROM nginx:alpine as ui-build
Copy ./web /usr/share/nginx/html

FROM rustlang/rust:nightly-slim as api-build

WORKDIR /usr/src/minimal-api
COPY . .
RUN cargo clean && cargo build --release

FROM debian:bullseye as api-release
ARG APP=/usr/src/app
RUN mkdir -p {$APP}

COPY --from=api-build /usr/src/minimal-api/target/release/minimal-ui-api-ms ${APP}/minimal-api
COPY --from=api-build /usr/src/minimal-api/Rocket.toml ${APP}/Rocket.toml

WORKDIR ${APP}

CMD ["./minimal-api"]
