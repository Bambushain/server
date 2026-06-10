ARG CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX

FROM $CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX/library/alpine:latest AS alpine

RUN apk add -U --no-cache ca-certificates

FROM scratch

WORKDIR /

COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY bamboo-public /bamboo
COPY ../../public /public

ENTRYPOINT ["/bamboo"]