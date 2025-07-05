FROM library/alpine:latest as alpine

RUN apk add -U --no-cache ca-certificates

FROM scratch

WORKDIR /app

COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY bamboo-public /app/bamboo
COPY public /app/

ENTRYPOINT ["/app/bamboo"]