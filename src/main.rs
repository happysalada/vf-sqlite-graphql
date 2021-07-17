use juniper::{http::graphiql, http::GraphQLRequest, EmptySubscription, RootNode};
use lazy_static::lazy_static;
use sqlx::sqlite::SqlitePool;
use trillium::{conn_try, Conn, Handler, Init, State};
use trillium_logger::Logger;
use trillium_router::Router;
mod schema;
use crate::schema::{MutationRoot, QueryRoot};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;
lazy_static! {
    static ref SCHEMA: Schema =
        Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new());
}

pub struct Context {
    pool: SqlitePool,
}

impl juniper::Context for Context {}

async fn handle_graphiql(conn: Conn) -> Conn {
    conn.with_header(("content-type", "text/html"))
        .ok(graphiql::graphiql_source("/graphql", None))
}

async fn handle_graphql(mut conn: Conn) -> Conn {
    let raw_body = conn_try!(conn, conn.request_body_string().await);
    let query: GraphQLRequest = conn_try!(conn, serde_json::from_str(&raw_body));
    let context = Context {
        pool: conn.state::<SqlitePool>().unwrap().to_owned(),
    };
    let response = query.execute(&SCHEMA, &context).await;
    let json = conn_try!(conn, serde_json::to_string(&response));
    conn.ok(json)
        .with_header(("Access-Control-Allow-Origin", "*"))
        .with_header(("content-type", "application/json"))
}

async fn cors(conn: Conn) -> Conn {
    conn.with_header(("Access-Control-Allow-Origin", "*"))
        .with_header(("Access-Control-Allow-Headers", "content-type"))
}

async fn not_found(conn: Conn) -> Conn {
    let body = format!("Uh oh, I don't have a route for {}", conn.path());
    conn.with_body(body).with_status(404)
}

pub fn application() -> impl Handler {
    env_logger::init();
    (
        Logger::new(),
        Init::new(|_| async move {
            let db = SqlitePool::connect(
                &std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
            )
            .await
            .expect("failed to get a db connection");
            State::new(db)
        }),
        cors,
        Router::new()
            .get("/graphiql", handle_graphiql)
            .post("/graphql", handle_graphql),
        not_found,
    )
}

fn main() {
    trillium_tokio::config()
        .with_port(
            std::env::var("HTTP_PORT")
                .expect("missing http port")
                .parse::<u16>()
                .expect("http port should be a number"),
        )
        .with_host("127.0.0.1")
        .with_nodelay()
        .run(application());
}

#[cfg(test)]
mod tests {
    use super::application;
    use trillium_testing::prelude::*;

    #[test]
    fn graphql_agents() {
        let mut application = application();
        init(&mut application);
        assert_response!(
            post("/graphql")
                .with_request_body(r#"{"query": "{ agents {id, name, email}}"}"#)
                .on(&application),
            StatusCode::Ok,
            r#"{"data":{"agents":[]}}"#,
        );
    }
}
