import sys
import asyncio
from itertools import chain

import dagger

HOSTNAMES = ["foo", "foo.test", "bar.baz.test"]


async def main():
    config = dagger.Config(log_output=sys.stdout)

    async with dagger.Connection(config) as client:
        # Build cli and server binaries
        rust_toolchain = client.host().file("rust-toolchain.toml")
        cli = build_rust_binary(
            client, client.host().directory("cli"), rust_toolchain, "cli"
        )
        server = build_rust_binary(
            client, client.host().directory("server"), rust_toolchain, "server"
        )

        # Setup containers for integration tests
        cli_con = client.container().from_("alpine").with_file("/bin/iqt", cli)
        for hostname in HOSTNAMES:
            cli_con = cli_con.with_service_binding(
                hostname, build_test_machines(client, server)
            )

        cli_con = cli_con.with_exec(
            [
                "/bin/iqt",
                "{hostname { name }}",
                *list(chain.from_iterable(["-f", h] for h in HOSTNAMES)),
            ]
        )
        out = await cli_con.stdout()
    print(out)


def build_rust_binary(
    client: dagger.Client,
    dir: dagger.Directory,
    rust_toolchain: dagger.File,
    name: str,
    target: str = "x86_64-unknown-linux-gnu",
) -> dagger.File:
    return (
        client.container()
        .from_("rust")
        .with_directory("/src", dir)
        .with_workdir("/src")
        .with_file("rust-toolchain.toml", rust_toolchain)
        .with_env_variable("RUSTFLAGS", "-C target-feature=+crt-static")
        .with_exec(
            [
                "cargo",
                "build",
                "--release",
                "--target",
                target,
            ]
        )
        .file(f"/src/target/{target}/release/{name}")
    )


def build_test_machines(client: dagger.Client, server: dagger.File) -> dagger.Container:
    return (
        client.container()
        .from_("alpine")
        .with_file("/bin/iqt_server", server)
        .with_exposed_port(8000)
        .with_entrypoint(["iqt_server"])
    )


if __name__ == "__main__":
    asyncio.run(main())
