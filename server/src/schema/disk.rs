use std::str::FromStr;

use anyhow::anyhow;
use async_graphql::{Object, SimpleObject};
use serde::Deserialize;
use tokio::process::Command;

pub struct Disk;

#[Object]
impl Disk {
    async fn usage(&self, path: Option<String>) -> anyhow::Result<Vec<DiskUsage>> {
        let mut cmd = Command::new("df");
        cmd.arg("-h");
        if let Some(path) = path {
            cmd.arg(path);
        }
        let output = cmd.output().await?;
        if !&output.status.success() {
            return match std::str::from_utf8(&output.stderr) {
                Ok(s) => Err(anyhow!("{}", s)),
                Err(e) => Err(e.into()),
            };
        }
        let output = String::from_utf8(output.stdout)?;
        parse_disk_usage(output)
    }
}

fn parse_disk_usage(input: String) -> anyhow::Result<Vec<DiskUsage>> {
    input
        .lines()
        .skip(1)
        .map(|line| DiskUsage::from_str(line.trim()))
        .collect()
}

#[derive(Deserialize, SimpleObject, Debug, PartialEq, Eq)]
struct DiskUsage {
    file_system: String,
    size: String,
    used: String,
    avalable: String,
    use_percent: u8,
    mount_point: String,
}

impl FromStr for DiskUsage {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 6 {
            return Err(anyhow!("Invalid Input"));
        }
        let use_percent: u8 = parts[4].trim_end_matches('%').parse()?;
        Ok(DiskUsage {
            file_system: parts[0].to_string(),
            size: parts[1].to_string(),
            used: parts[2].to_string(),
            avalable: parts[3].to_string(),
            use_percent,
            mount_point: parts[5].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage_from_str() {
        let input = "devtmpfs        790M     0  790M   0% /dev";
        let expected_disk_usage = DiskUsage {
            file_system: "devtmpfs".to_string(),
            size: "790M".to_string(),
            used: "0".to_string(),
            avalable: "790M".to_string(),
            use_percent: 0,
            mount_point: "/dev".to_string(),
        };

        let result = DiskUsage::from_str(input);
        assert_eq!(result.unwrap(), expected_disk_usage);
    }

    #[test]
    fn test_invalid_input_format() {
        let input = "invalid input format";
        let result = DiskUsage::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_use_percent() {
        let input = "example 100M 50M 50M invalid_percent /mount/point";
        let result = DiskUsage::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_example_input() {
        let input = "Filesystem      Size  Used Avail Use% Mounted on
        devtmpfs        790M     0  790M   0% /dev
        tmpfs           7.8G  1.1M  7.8G   1% /run/wrappers"
            .to_string();

        let expected = vec![
            DiskUsage {
                file_system: "devtmpfs".to_string(),
                size: "790M".to_string(),
                used: "0".to_string(),
                avalable: "790M".to_string(),
                use_percent: 0,
                mount_point: "/dev".to_string(),
            },
            DiskUsage {
                file_system: "tmpfs".to_string(),
                size: "7.8G".to_string(),
                used: "1.1M".to_string(),
                avalable: "7.8G".to_string(),
                use_percent: 1,
                mount_point: "/run/wrappers".to_string(),
            },
        ];

        let result = parse_disk_usage(input).unwrap();
        assert_eq!(result, expected);
    }
}
