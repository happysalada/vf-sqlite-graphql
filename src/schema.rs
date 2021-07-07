use crate::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use juniper::{graphql_object, FieldResult};
use ulid::Ulid;

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
}

#[derive(Clone, juniper::GraphQLObject)]
#[graphql(description = "A plan")]
struct Plan {
    id: String,
    title: String,
    description: Option<String>,
    agent_id: String,
    inserted_at: DateTime<Utc>,
}

#[derive(Clone, juniper::GraphQLObject)]
#[graphql(description = "An agent")]
struct Agent {
    id: String,
    name: String,
    unique_name: String,
    email: Option<String>,
    inserted_at: DateTime<Utc>,
}

#[derive(Clone)]
struct RawPlan {
    id: String,
    title: String,
    description: Option<String>,
    agent_id: String,
    inserted_at: i64,
}

impl RawPlan {
    fn to_graphql(self) -> Plan {
        Plan {
            id: self.id,
            title: self.title,
            description: self.description,
            agent_id: self.agent_id,
            inserted_at: timestamp_to_datetime(self.inserted_at),
        }
    }
}

#[derive(Clone)]
struct RawAgent {
    id: String,
    name: String,
    unique_name: String,
    email: Option<String>,
    inserted_at: i64,
}
impl RawAgent {
    fn to_graphql(self) -> Agent {
        Agent {
            id: self.id,
            name: self.name,
            unique_name: self.unique_name,
            email: self.email,
            inserted_at: timestamp_to_datetime(self.inserted_at),
        }
    }
}

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Plans")]
    async fn plans(context: &Context, agent_id: String) -> FieldResult<Vec<Plan>> {
        let plans = sqlx::query_as!(
            RawPlan,
            "SELECT * FROM plans WHERE plans.agent_id = ? ORDER BY inserted_at DESC",
            agent_id
        )
        .fetch_all(&context.pool)
        .await?;
        Ok(plans
            .iter()
            .cloned()
            .map(|raw_plan| raw_plan.to_graphql())
            .collect())
    }

    #[graphql(description = "Get all Agents")]
    async fn agents(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as!(RawAgent, "SELECT * FROM agents ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents
            .iter()
            .cloned()
            .map(|raw_agent| raw_agent.to_graphql())
            .collect())
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct NewPlan {
    title: String,
    agent_id: String,
    description: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct NewAgent {
    name: String,
    email: Option<String>,
}

pub struct MutationRoot;

#[graphql_object(Context=Context)]
impl MutationRoot {
    #[graphql(description = "Add new plan")]
    async fn create_plan(context: &Context, new_plan: NewPlan) -> FieldResult<Plan> {
        let ulid = Ulid::new().to_string();
        sqlx::query!(
            "INSERT INTO plans (id, title, agent_id) VALUES (?, ?, ?)",
            ulid,
            new_plan.title,
            new_plan.agent_id
        )
        .execute(&context.pool)
        .await?;
        let inserted_plan = sqlx::query_as!(RawPlan, "SELECT * FROM plans WHERE id = ?", ulid,)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_plan.to_graphql())
    }

    #[graphql(description = "Add new agent")]
    async fn create_agent(context: &Context, new_agent: NewAgent) -> FieldResult<Agent> {
        let ulid = Ulid::new().to_string();
        sqlx::query!(
            "INSERT INTO agents (id, name, unique_name, email) VALUES (?, ?, ?, ?)",
            ulid,
            new_agent.name,
            new_agent.name,
            new_agent.email
        )
        .execute(&context.pool)
        .await?;
        let inserted_agent = sqlx::query_as!(RawAgent, "SELECT * FROM agents WHERE id = ?", ulid,)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_agent.to_graphql())
    }
}
