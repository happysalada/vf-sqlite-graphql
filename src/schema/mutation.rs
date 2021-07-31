use super::{Agent, Label, Plan, Process};
use crate::Context;
use futures::future::join_all;
use juniper::{graphql_object, FieldResult};
use ulid::Ulid;

fn unique_name(name: String) -> String {
    name.to_lowercase().replace(" ", "_")
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct NewPlan {
    title: String,
    agent_id: String,
    description: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct UpdatePlan {
    id: String,
    title: String,
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
    plan_id: Option<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
}

#[derive(juniper::GraphQLInputObject, Debug)]
struct UpdateProcess {
    id: String,
    title: String,
    description: Option<String>,
    labels: Option<Vec<String>>,
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
        let inserted_plan = sqlx::query("SELECT * FROM plans WHERE id = ?")
            .bind(ulid)
            .map(Plan::from_row)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_plan)
    }

    #[graphql(description = "Update a plan")]
    async fn update_plan(context: &Context, update_plan: UpdatePlan) -> FieldResult<i32> {
        let result = sqlx::query("UPDATE plans SET title = ?, description = ? WHERE id = ?")
            .bind(update_plan.title)
            .bind(update_plan.description)
            .bind(update_plan.id)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    #[graphql(description = "Add new process")]
    async fn create_process(context: &Context, new_process: NewProcess) -> FieldResult<Process> {
        let ulid = Ulid::new().to_string();
        // TODO put those in a transaction
        sqlx::query("INSERT INTO processes (id, title, description, agent_id, plan_id) VALUES (?, ?, ?, ?, ?)")
            .bind(&ulid)
            .bind(new_process.title)
            .bind(new_process.description)
            .bind(new_process.agent_id)
            .bind(new_process.plan_id)
            .execute(&context.pool)
            .await?;
        // TODO paralelize queries
        let new_labels = new_process
            .labels
            .unwrap_or_else(Vec::new)
            .iter()
            .map(|label_id| {
                sqlx::query("INSERT INTO process_labels (process_id, label_id) VALUES (?, ?)")
                    .bind(&ulid)
                    .bind(label_id.clone())
                    .execute(&context.pool)
            })
            .collect::<Vec<_>>();
        join_all(new_labels).await;

        let mut inserted_process = sqlx::query("SELECT * FROM processes WHERE id = ?")
            .bind(&ulid)
            .map(Process::from_row)
            .fetch_one(&context.pool)
            .await?;
        let labels = sqlx::query("SELECT labels.id, name, color FROM labels INNER JOIN process_labels ON process_labels.label_id = labels.id WHERE process_labels.process_id = ?")
            .bind(ulid)
            .map(Label::from_row)
            .fetch_all(&context.pool)
            .await?;
        inserted_process.labels = labels;
        Ok(inserted_process)
    }

    #[graphql(description = "Update a process")]
    async fn update_process(context: &Context, update_process: UpdateProcess) -> FieldResult<i32> {
        // TODO paralelize queries
        let id: String = update_process.id;
        sqlx::query("DELETE FROM process_labels WHERE process_id = ?")
            .bind(&id)
            .execute(&context.pool)
            .await?;
        let new_labels = update_process
            .labels
            .unwrap_or_else(Vec::new)
            .iter()
            .map(|label_id| {
                sqlx::query("INSERT INTO process_labels (process_id, label_id) VALUES (?, ?)")
                    .bind(&id)
                    .bind(label_id.clone())
                    .execute(&context.pool)
            })
            .collect::<Vec<_>>();
        join_all(new_labels).await;

        let result = sqlx::query("UPDATE processes SET title = ?, description = ? WHERE id = ?")
            .bind(update_process.title)
            .bind(update_process.description)
            .bind(&id)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    #[graphql(description = "Delete a process")]
    async fn delete_process(context: &Context, process_id: String) -> FieldResult<i32> {
        let mut transaction = context.pool.begin().await?;
        sqlx::query("DELETE FROM process_labels WHERE process_id = ?")
            .bind(&process_id)
            .execute(&mut transaction)
            .await?;
        let result = sqlx::query("DELETE FROM processes WHERE id = ?")
            .bind(process_id)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected() as i32)
    }
}
