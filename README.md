# `iqt`

My Computer Science capstone project is a GraphQL server that allows system
administrators to remotely query information about the servers. When deployed
over multiple hosts, the second part will allow multiple host to be queried
collectively and aggregate the results.

# Major Components

The first - and primary - component is a GraphQL API that runs on each device.
It will be easily extendable with plugins - and a collection will be included
in the project. The service runs on port `4807` by default.

![](agent_overview.png) 

The second is a cli interface that allows users to query all the endpoints within
their network and aggregate the data. 

```bash
# Query all hosts in inventory file.
$ iqt '{query}' -i inventory

# Query all host on the specified subnet.
$ iqt '{quert}' -n 10.0.9.0/24 

# Query specific hosts
$ iqt '{query}' -h 10.0.9.2 10.0.9.3 
```

![](iqt_overview.png) 

# CI/CD

CI/CD is managed by Dagger. It builds release binaries for `cli` and `server`,
then constructs a test environment for integration testing. 

To run:

For now, the `dagger-io` python package isn't in `nixpkgs`, so it needs to be installed 
using pip.

```bash
python -m venv .venv
. .venv/bin/activate
pip install dagger-io
```
Then, run the CI using `dagger run python ci/main.py` or `python ci/main.py`.

# Resources

https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10#a-flake-with-a-dev-shell
