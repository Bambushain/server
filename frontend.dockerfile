FROM library/alpine:latest as alpine

RUN apk add -U --no-cache ca-certificates

FROM scratch

ARG APP
ARG PKG_DIR

ENV LEPTOS_ENV PROD

WORKDIR /

COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY $APP /bamboo
COPY $PKG_DIR /target/site

ENTRYPOINT ["/bamboo"]