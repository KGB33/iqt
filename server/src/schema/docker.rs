use anyhow::anyhow;
use async_graphql::{Object, SimpleObject};
use serde::Deserialize;
use tokio::process::Command;

pub struct Docker;

const DOCKER_FORMAT_ARGS: [&str; 2] = ["--format", "{{json .}}"];

/// Information
#[Object]
impl Docker {
    async fn ps(&self) -> anyhow::Result<Vec<DockerPsResponse>> {
        let output = Command::new("docker")
            .arg("ps")
            .args(DOCKER_FORMAT_ARGS)
            .output()
            .await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = std::str::from_utf8(&output.stdout)?;
        let mut processes: Vec<DockerPsResponse> = Vec::new();

        for line in output.lines() {
            let parsed = serde_json::from_str::<DockerPsResponse>(line)?;
            processes.push(parsed);
        }
        Ok(processes)
    }
}

#[derive(Deserialize, SimpleObject, Debug)]
#[serde(rename_all = "PascalCase")]
struct DockerPsResponse {
    command: String,
    created_at: String,
    image: String,
    names: String,
    running_for: String,
    state: String,
    status: String,
}
