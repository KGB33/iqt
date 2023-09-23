use std::error::Error;

use async_graphql::http::GraphiQLSource;
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};

mod schema;
use schema::schema;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = schema().unwrap();

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
