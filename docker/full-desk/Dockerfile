ARG COREVIA_DOCKER_BASE_IMAGE=debian:trixie-slim

FROM ${COREVIA_DOCKER_BASE_IMAGE}

ARG COREVIA_DESK_PORT=21116

ENV COREVIA_DESK_PROFILE=full-desk \
    COREVIA_DESK_PORT=${COREVIA_DESK_PORT} \
    COREVIA_ID_SERVER=87.107.8.155 \
    COREVIA_RELAY_SERVER=87.107.8.155 \
    COREVIA_API_SERVER= \
    COREVIA_SERVER_KEY=

RUN set -eu; \
    for source in /etc/apt/sources.list /etc/apt/sources.list.d/*.list /etc/apt/sources.list.d/*.sources; do \
        if [ -f "$source" ]; then \
            sed -i \
                -e 's|http://deb.debian.org|https://deb.debian.org|g' \
                -e 's|http://security.debian.org|https://security.debian.org|g' \
                "$source"; \
        fi; \
    done; \
    apt-get -o Acquire::https::Verify-Peer=false -o Acquire::https::Verify-Host=false update \
    && apt-get -o Acquire::https::Verify-Peer=false -o Acquire::https::Verify-Host=false install --yes --no-install-recommends ca-certificates \
    && apt-get update \
    && apt-get install --yes --no-install-recommends \
        libayatana-appindicator3-1 \
        libgtk-3-0 \
        libwebkit2gtk-4.1-0 \
        libxdo3 \
        xauth \
        xvfb \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/corevia
COPY CoreviaDesk /opt/corevia/CoreviaDesk
COPY web/public /opt/corevia/web/public
COPY entrypoint.sh /usr/local/bin/corevia-desk-entrypoint

RUN chmod 0755 /opt/corevia/CoreviaDesk /usr/local/bin/corevia-desk-entrypoint

# The default direct-access port can be replaced at runtime with
# COREVIA_DESK_PORT and a matching `-p host:container` mapping. ID/Relay
# ports can be supplied as host:port in COREVIA_ID_SERVER/COREVIA_RELAY_SERVER.
EXPOSE 21116/tcp 21116/udp 21117/tcp 21117/udp

ENTRYPOINT ["/usr/local/bin/corevia-desk-entrypoint"]
CMD ["--server"]
