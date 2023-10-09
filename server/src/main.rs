use std::error::Error;

use async_graphql::http::GraphiQLSource;
use async_graphql_poem::GraphQL;
use poem::{listener::TcpListener, web::Html, *};

mod schema;
use schema::generate_schema;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Route::new()
        .at("/", get(graphiql).post(GraphQL::new(generate_schema())))
        .at("/graphql", GraphQL::new(generate_schema()));
    println!("GraphiQL: http://localhost:4807");
    Server::new(TcpListener::bind("0.0.0.0:4807"))
        .run(app)
        .await?;
    Ok(())
}
