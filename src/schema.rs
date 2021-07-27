use crate::Context;
use futures::future::join_all;
use juniper::{graphql_object, FieldResult};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::collections::HashMap;
use std::default::Default;
use ulid::Ulid;

fn unique_name(name: String) -> String {
    name.to_lowercase().replace(" ", "_")
}

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
    fn from_join_row(row: SqliteRow) -> Self {
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
        let plans =
            sqlx::query("SELECT * FROM plans WHERE plans.agent_id = ? ORDER BY inserted_at DESC")
                .bind(agent_id)
                .map(Plan::from_row)
                .fetch_all(&context.pool)
                .await?;
        Ok(plans.to_vec())
    }

    #[graphql(description = "Get a Plan")]
    async fn plan(context: &Context, plan_id: String) -> FieldResult<Plan> {
        let mut plan = sqlx::query("SELECT * FROM plans WHERE plans.id = ?")
            .bind(plan_id)
            .map(Plan::from_row)
            .fetch_one(&context.pool)
            .await?;
        let mut processes = sqlx::query("SELECT * FROM processes WHERE processes.plan_id = ? ")
            .bind(&plan.id)
            .map(Process::from_row)
            .fetch_all(&context.pool)
            .await?;
        let labels_process_id = sqlx::query("SELECT labels.id, name, color, process_id FROM labels INNER JOIN process_labels ON process_labels.label_id = labels.id WHERE process_labels.process_id IN (SELECT id FROM processes WHERE processes.plan_id = ?)")
            .bind(&plan.id)
            .map(|row| (row.get("process_id"), Label::from_join_row(row)))
            .fetch_all(&context.pool)
            .await?;
        let plan_id_map: HashMap<String, Vec<Label>> = labels_process_id.iter().fold(
            HashMap::<String, Vec<Label>>::new(),
            |mut acc: HashMap<String, Vec<Label>>, (process_id, label): &(String, Label)| {
                let labels = acc.entry(process_id.to_owned()).or_insert_with(Vec::new);
                labels.push(label.clone());
                acc
            },
        );

        processes.iter_mut().for_each(|p| {
            p.labels = plan_id_map.get(&p.id).unwrap_or(&vec![]).clone();
        });
        plan.processes = processes;
        Ok(plan)
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
            .map(|row| Label{ id: row.get("id"), name: row.get("name"), color: row.get("color"), ..Default::default()})
            .fetch_all(&context.pool)
            .await?;
        inserted_process.labels = labels;
        Ok(inserted_process)
    }

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
