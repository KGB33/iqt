# Infrastructure Query Tool (`iqt`)

A read-only GraphQL API to query system information including Docker, IP, and
Hostname. The CLI allows users to query multiple systems and aggregate the
results.

## Build

Download the latest release from
[GitHub](https://github.com/KGB33/iqt/releases/). Or build from source using
[Dagger](https://dagger.io/):

```bash
# Build the Server
dagger -m github.com/KGB33/daggerverse/rust \
  -o iqt-server download \
  build --src server/ --name server --toolchain rust-toolchain.toml

# Build the cli
dagger -m github.com/KGB33/daggerverse/rust \
  -o iqt-cli download \
  build --src cli/ --name cli --toolchain rust-toolchain.toml
```

## Deploy

You can run local (`./iqt-server`), or - for a production setup - use a
Systemd unit file.

```ini
[Unit]
Description=IQT Server
After=network.target

[Service]
ExecStart=/usr/bin/iqt-server
Restart=always
DynamicUser=yes
SupplementaryGroups=docker

[Install]
WantedBy=default.target
```
## Usage

The interactive GraphQL query editor is available at
`http://<SERVER_IP>:4807/`, use this to see documentation and play with 
queries.

The CLI can be used to query multiple machines. The output is designed to be
piped into `jq`. 

```shell
./iqt-cli '{hostname {name} docker { ps { names state }}}' \
  -s 10.0.9.120 -f localhost \
  | jq '.[] | {(.data.hostname.name): .data.docker.ps[].names}'
{
  "minecraft": "atm9_mc_1"
}
{
  "localhost": "dagger-engine-3cb8ac0800e00d08"
}
```

Lastly, other applications can access the API by sending queries to `http://<SERVER_IP>:4807/graphql`.
