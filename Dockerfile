FROM library/alpine:latest as alpine

RUN apk add -U --no-cache ca-certificates

FROM scratch

ARG APP

WORKDIR /

COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY $APP /bamboo

ENTRYPOINT ["/bamboo"]