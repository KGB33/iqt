use async_graphql::{Enum, Object};
use tokio::process::Command;

pub struct Hostname;

#[Object]
impl Hostname {
    async fn name(&self, #[graphql(default)] flag: HostnameFlag) -> anyhow::Result<String> {
        let mut cmd = Command::new("hostname");
        match flag {
            HostnameFlag::Short => cmd.arg("--short"),
            HostnameFlag::Long => cmd.arg("--long"),
        };
        let output = cmd.output().await?;
        let hostname = std::str::from_utf8(&output.stdout)?;
        Ok(hostname.trim().to_string())
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)]
enum HostnameFlag {
    Short,
    #[default]
    Long,
}
