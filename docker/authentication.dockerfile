FROM registry.ulbricht.casa/docker-images/rust-docker-base-image:latest AS backend-authentication

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN cargo build --bin backend-authentication --features backend-authentication --release

FROM registry.ulbricht.casa/docker-images/trunk-docker-base-image:latest AS frontend-authentication

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN trunk build --config frontend-authentication.toml --release

FROM library/alpine:latest

ENV FRONTEND_DIR=/bamboo/authentication/

COPY --from=frontend-authentication dist-authentication /bamboo/authentication
COPY --from=backend-authentication /build/bamboo/target/release/backend-authentication /backend-authentication

CMD ["/backend-authentication"]
