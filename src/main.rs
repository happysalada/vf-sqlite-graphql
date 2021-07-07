use chrono::{DateTime, NaiveDateTime, Utc};
use juniper::{
    graphql_object, http::graphiql, http::GraphQLRequest, EmptySubscription, FieldResult, RootNode,
};
use lazy_static::lazy_static;
use sqlx::sqlite::SqlitePool;
use trillium::{conn_try, Conn, Handler, Init, State};
use trillium_logger::Logger;
use trillium_router::Router;
use ulid::Ulid;

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
}

#[derive(Clone)]
struct RawPlan {
    id: String,
    title: String,
    description: Option<String>,
    inserted_at: i64,
}

impl RawPlan {
    fn to_graphql(self) -> Plan {
        Plan {
            id: self.id,
            title: self.title,
            description: self.description,
            inserted_at: timestamp_to_datetime(self.inserted_at),
        }
    }
}

#[derive(Clone, juniper::GraphQLObject)]
#[graphql(description = "A plan")]
struct Plan {
    id: String,
    title: String,
    description: Option<String>,
    inserted_at: DateTime<Utc>,
}

#[derive(juniper::GraphQLInputObject)]
struct NewPlan {
    title: String,
    description: Option<String>,
}

impl NewPlan {
    fn into_internal(self) -> Plan {
        Plan {
            id: Ulid::new().to_string(),
            title: self.title,
            description: self.description,
            inserted_at: Utc::now(),
        }
    }
}

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Plans")]
    async fn plans(context: &Context) -> FieldResult<Vec<Plan>> {
        let plans = sqlx::query_as!(RawPlan, "SELECT * FROM plans ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(plans
            .iter()
            .cloned()
            .map(|raw_plan| raw_plan.to_graphql())
            .collect())
    }
}

pub struct MutationRoot;

#[graphql_object(Context=Context)]
impl MutationRoot {
    #[graphql(description = "Add new plan")]
    async fn create_plan(context: &Context, plan: NewPlan) -> FieldResult<Plan> {
        let ulid = Ulid::new().to_string();
        sqlx::query!(
            "INSERT INTO PLANS (id, title) VALUES (?, ?)",
            ulid,
            plan.title
        )
        .execute(&context.pool)
        .await?;
        Ok(plan.into_internal())
    }
}

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

async fn handle_graphql_options(conn: Conn) -> Conn {
    conn.ok("All good")
        .with_header(("Access-Control-Allow-Origin", "*"))
}

async fn not_found(conn: Conn) -> Conn {
    let body = format!("Uh oh, I don't have a route for {}", conn.path());
    conn.with_body(body).with_status(404)
}

pub fn application() -> impl Handler {
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
        Router::new()
            .get("/graphiql", handle_graphiql)
            .post("/graphql", handle_graphql)
            .any("/graphql", handle_graphql_options),
        not_found,
    )
}

fn main() {
    env_logger::init();
    trillium_tokio::config()
        .with_port(8080)
        .with_host("127.0.0.1")
        .with_nodelay()
        .run(application());
}

#[cfg(test)]
mod tests {
    use super::application;
    use log::{error, info};
    use trillium_testing::prelude::*;

    #[test]

    fn graphql_plans() {
        let application = application();
        assert_response!(
            post("/graphql")
                .with_request_body("query { plans { id } }")
                .on(&application),
            StatusCode::Ok,
            "data: []",
        );
    }

    #[test]
    fn graphql_create_plan() {
        let application = application();
        let db_url = std::env::var("DATABASE_URL").expect("missing database url");
        error!("DATABASE_URL {}", db_url);
        assert_response!(
            post("/graphql")
                .with_request_body("query { plans { id } }")
                .on(&application),
            StatusCode::Ok,
            "data: []",
        );
    }

    #[test]
    fn graphql_delete_plan() {
        let application = application();
        let db_url = std::env::var("DATABASE_URL").expect("missing database url");
        info!("DATABASE_URL {}", db_url);
        assert_response!(
            post("/graphql")
                .with_request_body("query { plans { id } }")
                .on(&application),
            StatusCode::Ok,
            "data: []",
        );
    }
}
