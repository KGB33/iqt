use async_graphql::{EmptyMutation, EmptySubscription, Enum, Object, Schema};
use tokio::process::Command;

pub fn generate_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

#[derive(Debug)]
pub struct Query;

#[Object]
impl Query {
    async fn hostname(&self) -> Hostname {
        Hostname
    }
}

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
