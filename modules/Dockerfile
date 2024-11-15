FROM rust:slim-bookworm as base
ARG MODULE
ARG OPTIONAL_DEPS
ARG ADDITIONAL_PACKAGES="curl ca-certificates libpq-dev"

WORKDIR /${MODULE}

RUN --mount=type=bind,source=./modules/${MODULE},target=/${MODULE}/,readwrite\
 rm -f /etc/apt/apt.conf.d/docker-clean \
 && apt-get update \
 && DEBIAN_FRONTEND=noninteractive\
    apt-get install --no-install-recommends --assume-yes ${ADDITIONAL_PACKAGES} \
 && cargo install --path .

FROM scratch as release
ARG MODULE
COPY ./${MODULE}/target/x86_64-unknown-linux-musl/release/${MODULE_NAME} /app
ENTRYPOINT ["/app"]
