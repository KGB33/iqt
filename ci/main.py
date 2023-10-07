from dataclasses import dataclass
import sys
import asyncio

import dagger

FLAKE_FLAG = [
    "--extra-experimental-features",
    "nix-command",
    "--extra-experimental-features",
    "flakes",
]

HOSTNAMES = ["foo", "foo.test", "bar.baz.test"]


@dataclass
class Flake:
    nix: dagger.File
    lock: dagger.File
    rust_toolchain: dagger.File


async def main():
    config = dagger.Config(log_output=sys.stdout)

    async with dagger.Connection(config) as client:
        flake = Flake(
            client.host().file("flake.nix"),
            client.host().file("flake.lock"),
            client.host().file("rust-toolchain.toml"),
        )
        bin = build_rust_binary(client, client.host().directory("."), flake)

        remotes = [
            build_test_machines(client, bin.file("/src/target/debug/server"), differ)
            for differ in ("1", "2", "3")
        ]

        cli_con = (
            client.container()
            .from_("ubuntu:23.10")
            .with_file("/bin/iqt", bin.file("/src/target/debug/cli"))
        )
        for hostname, remote in zip(HOSTNAMES, remotes):
            cli_con = cli_con.with_service_binding(hostname, remote)

        cli_con = cli_con.with_exec(
            ["/bin/iqt", "'{hostname { name }}'", "-s", "'127.0.0.1'"]
        )
        out = await cli_con.stdout()
    print(out)


def build_rust_binary(
    client: dagger.Client, dir: dagger.Directory, flake: Flake
) -> dagger.Container:
    cmd_prexfix = ["nix", "develop", *FLAKE_FLAG, "--command"]
    return (
        client.container()
        .from_("nixos/nix")
        .with_directory("/src", dir)
        .with_file("/src/flake.lock", flake.lock)
        .with_file("/src/flake.nix", flake.nix)
        .with_file("/src/rust-toolchain.toml", flake.rust_toolchain)
        .with_workdir("/src")
        .with_exec([*cmd_prexfix, "cargo", "build", "--workspace"])
    )


def build_test_machines(
    client: dagger.Client, server: dagger.File, differ: str
) -> dagger.Container:
    return (
        client.container()
        .from_("ubuntu:23.10")
        .with_env_variable("DIFFER", differ)
        .with_file("/bin/iqt_server", server)
        .with_exposed_port(8000)
        .with_entrypoint(["iqt_server"])
    )


if __name__ == "__main__":
    asyncio.run(main())
