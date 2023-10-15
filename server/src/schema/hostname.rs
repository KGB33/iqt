use async_graphql::{Object, Enum};
use tokio::process::Command;

pub struct Hostname;

#[Object]
impl Hostname {
    async fn name(&self, #[graphql(default)] flag: HostnameFlag) -> String {
        let mut cmd = Command::new("hostname");
        match flag {
            HostnameFlag::Short => cmd.arg("--short"),
            HostnameFlag::Long => cmd.arg("--long"),
        };
        let output = match cmd.output().await {
            Ok(o) => o,
            Err(e) => return e.to_string(),
        };
        match std::str::from_utf8(&output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(e) => e.to_string(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)]
enum HostnameFlag {
    Short,
    #[default]
    Long,
}
