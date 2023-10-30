use anyhow::anyhow;
use async_graphql::{Object, SimpleObject};
use serde::Deserialize;
use tokio::process::Command;

pub struct Ip;

/// Maps to `iproute2`'s `ip` command
#[Object]
impl Ip {
    /// `ip route ...`
    async fn route(&self) -> Route {
        Route
    }

    /// `ip route adress...`
    async fn address(&self) -> Address {
        Address
    }

    /// `ip route link ...`
    async fn link(&self) -> Link {
        Link
    }
}

// *** Route ***
struct Route;

#[Object]
impl Route {
    /// Lists all the routes on the system
    async fn list(&self) -> anyhow::Result<Vec<RouteInfo>> {
        let output = Command::new("ip")
            .arg("-j")
            .arg("route")
            .arg("list")
            .output()
            .await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = std::str::from_utf8(&output.stdout)?;
        match serde_json::from_str(&output) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Describes the route to get to the provided Ip Address
    async fn get(&self, ip_address: String) -> anyhow::Result<Vec<RouteInfo>> {
        let output = Command::new("ip")
            .arg("-j")
            .arg("route")
            .arg("get")
            .arg(ip_address)
            .output()
            .await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = std::str::from_utf8(&output.stdout)?;
        match serde_json::from_str(&output) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(SimpleObject, Deserialize)]
struct RouteInfo {
    dst: String,
    gateway: Option<String>,
    dev: String,
    protocol: Option<String>,
    prefsrc: String,
    metric: Option<u32>,
    flags: Vec<String>,
}

// *** Address ***
struct Address;

#[Object]
impl Address {
    /// Gets Ip Address information
    async fn show(&self, link: Option<String>) -> anyhow::Result<Vec<AddressInfo>> {
        let mut cmd = Command::new("ip");
        cmd.arg("-j").arg("address").arg("show");
        if let Some(link) = link {
            cmd.arg(link);
        }
        let output = cmd.output().await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = std::str::from_utf8(&output.stdout)?;
        match serde_json::from_str(&output) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(SimpleObject, Deserialize)]
struct AddressInfo {
    ifindex: u32,
    ifname: String,
    flags: Vec<String>,
    mtu: u32, // Max MTU on loopback is u16+1 :(
    qdisc: String,
    operstate: String,
    group: String,
    txqlen: Option<u16>,
    link_type: String,
    address: String,
    broadcast: String,
    addr_info: Vec<AddrInfo>,
}

#[derive(SimpleObject, Deserialize)]
struct AddrInfo {
    family: String,
    local: String,
    prefixlen: u16,
    scope: String,
    label: Option<String>,
    noprefixrotue: Option<bool>,
    valid_life_time: u32,
    preferred_life_time: u32,
}

// *** Link ***
struct Link;

#[Object]
impl Link {
    async fn show(
        &self,
        #[graphql(default)] link: Option<String>,
    ) -> anyhow::Result<Vec<LinkInfo>> {
        let mut cmd = Command::new("ip");
        cmd.arg("-j").arg("link").arg("show");
        if let Some(link) = link {
            cmd.arg(link);
        }
        let output = cmd.output().await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = std::str::from_utf8(&output.stdout)?;
        match serde_json::from_str(&output) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(SimpleObject, Deserialize)]
struct LinkInfo {
    ifindex: u32,
    ifname: String,
    flags: Vec<String>,
    mtu: u32,
    qdisc: String,
    operstate: String,
    linkmode: String,
    group: String,
    txqlen: Option<u16>,
    link_type: String,
    address: String,
    broadcast: String,
}
