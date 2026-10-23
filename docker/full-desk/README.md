# Corevia Desk Full Desk — Linux container

This image contains the Full Desk Rust runtime. It is built only from the
Linux Full Desk release bundle; the Customer Support profile is not included.
The base image is resolved through the internal Nexus Docker group; the
release path does not pull a base image directly from Docker Hub.

The container reads its network settings before the Rust runtime starts:

```sh
docker run --rm \
  -e COREVIA_ID_SERVER=87.107.8.155:21116 \
  -e COREVIA_RELAY_SERVER=87.107.8.155:21117 \
  -e COREVIA_SERVER_KEY='<server-public-key>' \
  -e COREVIA_DESK_PORT=22000 \
  -p 22000:22000/tcp \
  -p 22000:22000/udp \
  corevia-desk/full-desk-linux:VERSION
```

`COREVIA_API_SERVER` is optional. `COREVIA_ID_SERVER` and
`COREVIA_RELAY_SERVER` may include their own `:port`. `COREVIA_DESK_PORT`
controls the local direct-access port and is validated by the entrypoint.

The normal command is `--server`. A different explicitly supported Corevia
Desk argument can be supplied after the image name. The image uses Xvfb when
no display is provided, which makes it suitable for a headless Linux host;
screen-sharing still depends on the host permissions and display integration.
