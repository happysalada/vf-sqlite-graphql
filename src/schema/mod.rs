use async_graphql::{EmptySubscription, Enum, Schema, SimpleObject};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::default::Default;
pub mod mutation;
pub mod query;
pub use mutation::MutationRoot;
pub use query::QueryRoot;

pub type VfSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject, Default)]
struct Plan {
    id: String,
    title: String,
    description: Option<String>,
    processes: Vec<Process>,
    inserted_at: String,
}

impl Plan {
    fn from_row(row: SqliteRow) -> Self {
        Plan {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            inserted_at: row.get("inserted_at"),
            ..Default::default()
        }
    }
}

#[derive(sqlx::Type, Copy, Clone, Debug, Eq, PartialEq, Enum)]
enum AgentType {
    Individual,
    Organization,
    Project,
}
impl Default for AgentType {
    fn default() -> Self {
        AgentType::Individual
    }
}

#[derive(Clone, SimpleObject, FromRow, Debug, Default)]
struct Agent {
    id: String,
    name: String,
    unique_name: String,
    email: Option<String>,
    agent_type: AgentType,
    inserted_at: i64,
}

impl Agent {
    fn from_row(row: SqliteRow) -> Self {
        Agent {
            id: row.get("id"),
            name: row.get("name"),
            unique_name: row.get("unique_name"),
            agent_type: row.get("agent_type"),
            ..Default::default()
        }
    }
}

#[derive(Clone, SimpleObject, FromRow, Debug, Default)]
struct AgentRelationship {
    id: String,
    subject_id: String,
    subject: Agent,
    object_id: String,
    object: Agent,
    agent_relation_type_id: String,
    agent_relation_type: String,
    inserted_at: String,
}

impl AgentRelationship {
    fn from_row(row: SqliteRow) -> Self {
        AgentRelationship {
            id: row.get("id"),
            subject_id: row.get("subject_id"),
            object_id: row.get("object_id"),
            agent_relation_type: row.get("agent_relation_type_name"),
            ..Default::default()
        }
    }
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
struct Label {
    id: String,
    name: String,
    unique_name: String,
    color: Option<String>,
    inserted_at: i64,
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

#[derive(Clone, SimpleObject, Default, Debug)]
struct Process {
    id: String,
    title: String,
    description: Option<String>,
    labels: Vec<Label>,
    agents: Vec<Agent>,
    inserted_at: String,
    start_at: String,
    due_at: String,
    plan_id: String,
    agent_id: String,
    commitments: Vec<Commitment>,
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

#[derive(sqlx::Type, Clone, Debug, Copy, Eq, PartialEq, Enum)]
enum InputOutput {
    Input,
    Output,
}
impl Default for InputOutput {
    fn default() -> Self {
        InputOutput::Input
    }
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
struct Action {
    id: String,
    name: String,
    input_output: InputOutput,
    inserted_at: String,
}

impl Action {
    fn from_row(row: SqliteRow) -> Self {
        Action {
            id: row.get("id"),
            name: row.get("name"),
            input_output: row.get("input_output"),
            inserted_at: row.get("inserted_at"),
        }
    }
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
struct Unit {
    id: String,
    label: String,
    inserted_at: i64,
}

impl Unit {
    fn from_row(row: SqliteRow) -> Self {
        Unit {
            id: row.get("id"),
            label: row.get("label"),
            inserted_at: row.get("inserted_at"),
        }
    }
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
struct ResourceSpecification {
    id: String,
    name: String,
    unique_name: String,
    inserted_at: i64,
}

impl ResourceSpecification {
    fn from_row(row: SqliteRow) -> Self {
        ResourceSpecification {
            id: row.get("id"),
            name: row.get("name"),
            inserted_at: row.get("inserted_at"),
            ..Default::default()
        }
    }
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
struct Commitment {
    id: String,
    description: String,
    process_id: String,
    action_id: String,
    action: Option<Action>,
    assigned_agent_id: Option<String>,
    assigned_agent: Option<Agent>,
    quantity: i32,
    unit_id: String,
    unit: Option<Unit>,
    resource_specification_id: String,
    resource_specification: Option<ResourceSpecification>,
    inserted_at: String,
}

impl Commitment {
    fn from_row(row: SqliteRow) -> Self {
        Commitment {
            id: row.get("id"),
            description: row.get("description"),
            inserted_at: row.get("inserted_at"),
            process_id: row.get("process_id"),
            action_id: row.get("action_id"),
            assigned_agent_id: row.get("assigned_agent_id"),
            quantity: row.get("quantity"),
            unit_id: row.get("unit_id"),
            resource_specification_id: row.get("resource_specification_id"),
            ..Default::default()
        }
    }
}
