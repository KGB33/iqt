use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

mod hostname;
use hostname::Hostname;

mod ip;
use ip::Ip;

mod docker;
use docker::Docker;

mod disk;
use disk::Disk;

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

    async fn ip(&self) -> Ip {
        Ip
    }

    async fn docker(&self) -> Docker {
        Docker
    }

    async fn disk(&self) -> Disk {
        Disk
    }
}
