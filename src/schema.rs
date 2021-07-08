use crate::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use juniper::{graphql_object, FieldResult};
use ulid::Ulid;

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
}

fn unique_name(name: String) -> String {
    name.to_lowercase().replace(" ", "_")
}

#[derive(Clone, juniper::GraphQLObject, sqlx::FromRow)]
#[graphql(description = "A plan")]
struct Plan {
    id: String,
    title: String,
    description: Option<String>,
    agent_id: String,
    inserted_at: String,
}

#[derive(Clone, juniper::GraphQLObject, sqlx::FromRow)]
#[graphql(description = "An agent")]
struct Agent {
    id: String,
    name: String,
    unique_name: String,
    email: Option<String>,
    inserted_at: String,
}

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Plans")]
    async fn plans(context: &Context, agent_id: String) -> FieldResult<Vec<Plan>> {
        let plans = sqlx::query_as::<_, Plan>(
            "SELECT * FROM plans WHERE plans.agent_id = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_id)
        .fetch_all(&context.pool)
        .await?;
        Ok(plans.to_vec())
    }

    #[graphql(description = "Get all Agents")]
    async fn agents(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents.to_vec())
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
        sqlx::query("INSERT INTO plans (id, title, agent_id) VALUES (?, ?, ?)")
            .bind(&ulid)
            .bind(new_plan.title)
            .bind(new_plan.agent_id)
            .execute(&context.pool)
            .await?;
        let inserted_plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE id = ?")
            .bind(ulid)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_plan)
    }

    #[graphql(description = "Add new agent")]
    async fn create_agent(context: &Context, new_agent: NewAgent) -> FieldResult<Agent> {
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(new_agent.name.clone());
        sqlx::query("INSERT INTO agents (id, name, unique_name, email) VALUES (?, ?, ?, ?)")
            .bind(&ulid)
            .bind(new_agent.name)
            .bind(unique_name)
            .bind(new_agent.email)
            .execute(&context.pool)
            .await?;
        let inserted_agent = sqlx::query_as::<_, Agent>("SELECT * FROM agents WHERE id = ?")
            .bind(ulid)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_agent)
    }
}
