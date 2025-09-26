FROM ${CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX}/library/alpine:latest as alpine

RUN apk add -U --no-cache ca-certificates

FROM scratch

ARG APP
ARG PKG_DIR

ENV LEPTOS_ENV PROD

WORKDIR /

COPY --from=${CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX}/alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY $APP /bamboo
COPY $PKG_DIR /target/site

ENTRYPOINT ["/bamboo"]