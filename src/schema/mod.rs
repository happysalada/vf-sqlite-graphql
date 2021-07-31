use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::default::Default;
pub mod mutation;
pub mod query;
pub use mutation::MutationRoot;
pub use query::QueryRoot;

#[derive(Clone, juniper::GraphQLObject, Default)]
#[graphql(description = "A plan")]
struct Plan {
    id: String,
    title: String,
    description: Option<String>,
    agent_id: String,
    processes: Vec<Process>,
    inserted_at: String,
}

impl Plan {
    fn from_row(row: SqliteRow) -> Self {
        Plan {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            agent_id: row.get("agent_id"),
            inserted_at: row.get("inserted_at"),
            ..Default::default()
        }
    }
}

#[derive(Clone, juniper::GraphQLObject, FromRow)]
#[graphql(description = "An agent")]
struct Agent {
    id: String,
    name: String,
    unique_name: String,
    email: Option<String>,
    inserted_at: String,
}

#[derive(Clone, juniper::GraphQLObject, Debug, Default, FromRow)]
#[graphql(description = "A label")]
struct Label {
    id: String,
    name: String,
    unique_name: String,
    color: String,
    inserted_at: String,
    agent_id: String,
}

impl Label {
    fn from_row(row: SqliteRow) -> Self {
        Label {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            ..Default::default()
        }
    }
}

#[derive(Clone, juniper::GraphQLObject, Default, Debug)]
#[graphql(description = "A process")]
struct Process {
    id: String,
    title: String,
    description: Option<String>,
    labels: Vec<Label>,
    inserted_at: String,
    start_at: String,
    due_at: String,
    plan_id: String,
    agent_id: String,
}

impl Process {
    fn from_row(row: SqliteRow) -> Self {
        Process {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            inserted_at: row.get("inserted_at"),
            start_at: row.get("start_at"),
            plan_id: row.get("plan_id"),
            ..Default::default()
        }
    }
}

#[derive(sqlx::Type, Clone, Debug, juniper::GraphQLEnum)]
enum InputOutput {
    Input,
    Output,
}
impl Default for InputOutput {
    fn default() -> Self {
        InputOutput::Input
    }
}

#[derive(Clone, juniper::GraphQLObject, Debug, Default, FromRow)]
#[graphql(description = "An action")]
struct Action {
    id: String,
    name: String,
    input_output: InputOutput,
    inserted_at: String,
}

#[derive(Clone, juniper::GraphQLObject, Debug, Default, FromRow)]
#[graphql(description = "A unit of measure")]
struct Unit {
    id: String,
    label: String,
    inserted_at: String,
}
