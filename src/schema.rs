use crate::Context;
use juniper::{graphql_object, FieldResult};
use ulid::Ulid;

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

#[derive(Clone, juniper::GraphQLObject, sqlx::FromRow)]
#[graphql(description = "A label")]
struct Label {
    id: String,
    name: String,
    unique_name: String,
    color: String,
    inserted_at: String,
    agent_id: String,
}

#[derive(Clone, juniper::GraphQLObject, sqlx::FromRow)]
#[graphql(description = "A process")]
struct Process {
    id: String,
    title: String,
    description: Option<String>,
    inserted_at: String,
    start_at: String,
    due_at: String,
    plan_id: String,
    agent_id: String,
}

pub struct QueryRoot;

#[graphql_object(Context=Context)]
impl QueryRoot {
    #[graphql(description = "Get all Agents")]
    async fn agents(context: &Context) -> FieldResult<Vec<Agent>> {
        let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents ORDER BY inserted_at DESC")
            .fetch_all(&context.pool)
            .await?;
        Ok(agents.to_vec())
    }

    #[graphql(description = "Get all Plans for an agent")]
    async fn plans(context: &Context, agent_id: String) -> FieldResult<Vec<Plan>> {
        let plans = sqlx::query_as::<_, Plan>(
            "SELECT * FROM plans WHERE plans.agent_id = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_id)
        .fetch_all(&context.pool)
        .await?;
        Ok(plans.to_vec())
    }

    #[graphql(description = "Get all labels for an agent")]
    async fn labels(context: &Context, agent_id: String) -> FieldResult<Vec<Label>> {
        let labels = sqlx::query_as::<_, Label>(
            "SELECT * FROM labels WHERE labels.agent_id = ? ORDER BY inserted_at DESC",
        )
        .bind(agent_id)
        .fetch_all(&context.pool)
        .await?;
        Ok(labels.to_vec())
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

#[derive(juniper::GraphQLInputObject, Debug)]
struct NewLabel {
    name: String,
    color: String,
    agent_id: String,
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct NewProcess {
    title: String,
    description: Option<String>,
    agent_id: String,
    start_date: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<NewLabel>>,
}

pub struct MutationRoot;

#[graphql_object(Context=Context)]
impl MutationRoot {
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

    async fn delete_agent(context: &Context, unique_name: String) -> FieldResult<i32> {
        let result = sqlx::query("DELETE FROM agents WHERE unique_name = ?")
            .bind(unique_name)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    #[graphql(description = "Add a new label")]
    async fn create_label(context: &Context, new_label: NewLabel) -> FieldResult<Label> {
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(new_label.name.clone());
        sqlx::query(
            "INSERT INTO labels (id, name, unique_name, color, agent_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&ulid)
        .bind(new_label.name)
        .bind(unique_name)
        .bind(new_label.color)
        .bind(new_label.agent_id)
        .execute(&context.pool)
        .await?;
        let inserted_label = sqlx::query_as::<_, Label>("SELECT * FROM labels WHERE id = ?")
            .bind(ulid)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_label)
    }

    async fn delete_label(context: &Context, unique_name: String) -> FieldResult<i32> {
        let result = sqlx::query("DELETE FROM labels WHERE unique_name = ?")
            .bind(unique_name)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

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

    #[graphql(description = "Add new process")]
    async fn create_process(context: &Context, new_process: NewProcess) -> FieldResult<Process> {
        dbg!(&new_process);
        let ulid = Ulid::new().to_string();
        sqlx::query("INSERT INTO processes (id, title, description, agent_id) VALUES (?, ?, ?, ?)")
            .bind(&ulid)
            .bind(new_process.title)
            .bind(new_process.description)
            .bind(new_process.agent_id)
            .execute(&context.pool)
            .await?;
        let inserted_process = sqlx::query_as::<_, Process>("SELECT * FROM processes WHERE id = ?")
            .bind(ulid)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_process)
    }
}
