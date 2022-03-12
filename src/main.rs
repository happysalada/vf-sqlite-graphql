use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    handler::Handler,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod schema;
use crate::schema::{MutationRoot, QueryRoot, VfSchema};

const GRAPHQL_URL: &str = "/graphql";

async fn graphql_handler(schema: Extension<VfSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new(GRAPHQL_URL).subscription_endpoint("/ws"),
    ))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let db = SqlitePool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"))
        .await
        .expect("failed to get a db connection");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a route
    let app = Router::new()
        .route(GRAPHQL_URL, get(graphql_playground).post(graphql_handler))
        .layer(cors)
        .layer(Extension(schema))
        .fallback(not_found.into_service());

    let port = std::env::var("HTTP_PORT")
        .expect("missing http port")
        .parse::<u16>()
        .expect("http port should be a number");
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[cfg(all(test, feature = "with-db"))]
mod tests {
    use super::application;

    #[test]
    fn graphql_agents() {
        let mut application = application();
        init(&mut application);
        assert_response!(
            post("/graphql")
                .with_request_body(r#"{"query": "{ agents {id, name, email}}"}"#)
                .on(&application),
            Status::Ok,
            r#"{"data":{"agents":[]}}"#,
        );
    }
}
