use super::{Action, Agent, Commitment, Label, Plan, Process, ResourceSpecification, Unit};
use crate::Context;
use futures::future::join_all;
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use ulid::Ulid;

fn unique_name(name: &str) -> String {
    name.to_string().to_lowercase().replace(" ", "_")
}

#[derive(GraphQLInputObject, Debug)]
struct NewPlan {
    title: String,
    agent_unique_name: String,
    description: Option<String>,
}

#[derive(GraphQLInputObject, Debug)]
struct UpdatePlan {
    id: String,
    title: String,
    description: Option<String>,
}

#[derive(GraphQLInputObject, Debug)]
struct NewLabel {
    name: String,
    color: String,
    agent_unique_name: String,
}

#[derive(GraphQLInputObject, Debug)]
struct NewProcess {
    title: String,
    description: Option<String>,
    plan_id: Option<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
    agents: Option<Vec<String>>,
}

#[derive(GraphQLInputObject, Debug)]
struct UpdateProcess {
    id: String,
    title: String,
    description: Option<String>,
    labels: Option<Vec<String>>,
    agents: Option<Vec<String>>,
}

#[derive(GraphQLInputObject, Debug)]
struct NewResourceSpecification {
    name: String,
    agent_unique_name: String,
}

#[derive(GraphQLInputObject, Debug)]
struct NewCommitment {
    description: String,
    process_id: String,
    action_id: String,
    assigned_agent_id: Option<String>,
    resource_specification_id: String,
    quantity: i32,
    unit_id: String,
    due_at: Option<String>,
}

#[derive(GraphQLInputObject, Debug)]
struct UpdateCommitment {
    id: String,
    description: Option<String>,
    action_id: Option<String>,
    quantity: Option<i32>,
    unit_id: Option<String>,
    resource_specification_id: Option<String>,
    assigned_agent_id: Option<String>,
    due_at: Option<String>,
}

pub struct MutationRoot;

#[graphql_object(Context=Context)]
impl MutationRoot {
    #[graphql(description = "Add a new label")]
    async fn create_label(context: &Context, new_label: NewLabel) -> FieldResult<Label> {
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(&new_label.name);
        sqlx::query(
            "INSERT INTO labels (id, name, unique_name, color, agent_unique_name) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&ulid)
        .bind(new_label.name)
        .bind(unique_name)
        .bind(new_label.color)
        .bind(new_label.agent_unique_name)
        .execute(&context.pool)
        .await?;
        let inserted_label = sqlx::query_as::<_, Label>("SELECT * FROM labels WHERE id = ?")
            .bind(ulid)
            .fetch_one(&context.pool)
            .await?;
        Ok(inserted_label)
    }

    #[graphql(description = "Delete a label")]
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
        sqlx::query("INSERT INTO plans (id, title) VALUES (?, ?)")
            .bind(&ulid)
            .bind(new_plan.title)
            .execute(&context.pool)
            .await?;
        sqlx::query("INSERT INTO plan_agents (plan_id, agent_unique_name) VALUES (?, ?)")
            .bind(&ulid)
            .bind(new_plan.agent_unique_name)
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
        sqlx::query(
            "
            INSERT INTO processes (id, title, description, plan_id)
            VALUES (?, ?, ?, ?)
            ",
        )
        .bind(&ulid)
        .bind(new_process.title)
        .bind(new_process.description)
        .bind(new_process.plan_id)
        .execute(&context.pool)
        .await?;
        // TODO paralelize queries
        let new_process_labels = new_process
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

        let new_process_agents = new_process
            .agents
            .unwrap_or_else(Vec::new)
            .iter()
            .map(|agent_id| {
                sqlx::query("INSERT INTO process_agents (process_id, agent_id) VALUES (?, ?)")
                    .bind(&ulid)
                    .bind(agent_id.clone())
                    .execute(&context.pool)
            })
            .collect::<Vec<_>>();
        // TODO parallelize
        join_all(new_process_labels).await;
        join_all(new_process_agents).await;

        let mut inserted_process = sqlx::query("SELECT * FROM processes WHERE id = ?")
            .bind(&ulid)
            .map(Process::from_row)
            .fetch_one(&context.pool)
            .await?;
        let labels = sqlx::query(
            "
           SELECT labels.id, name, color
           FROM labels
           INNER JOIN process_labels
           ON process_labels.label_id = labels.id
           WHERE process_labels.process_id = ?
           ",
        )
        .bind(&ulid)
        .map(Label::from_row)
        .fetch_all(&context.pool)
        .await?;
        let agents = sqlx::query(
            "
            SELECT agents.id, name, unique_name
            FROM agents
            INNER JOIN process_agents
            ON process_agents.agent_id = agents.id
            WHERE process_agents.process_id = ?
            ",
        )
        .bind(ulid)
        .map(Agent::from_row)
        .fetch_all(&context.pool)
        .await?;
        inserted_process.labels = labels;
        inserted_process.agents = agents;
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
        sqlx::query("DELETE FROM process_agents WHERE process_id = ?")
            .bind(&id)
            .execute(&context.pool)
            .await?;
        let new_process_labels = update_process
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
        let new_process_agents = update_process
            .agents
            .unwrap_or_else(Vec::new)
            .iter()
            .map(|agent_id| {
                sqlx::query("INSERT INTO process_agents (process_id, agent_id) VALUES (?, ?)")
                    .bind(&id)
                    .bind(agent_id.clone())
                    .execute(&context.pool)
            })
            .collect::<Vec<_>>();
        join_all(new_process_labels).await;
        join_all(new_process_agents).await;

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
        sqlx::query("DELETE FROM process_agents WHERE process_id = ?")
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

    #[graphql(description = "Add a new resource specification")]
    async fn create_resource_specification(
        context: &Context,
        new_resource_specification: NewResourceSpecification,
    ) -> FieldResult<ResourceSpecification> {
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(&new_resource_specification.name);
        sqlx::query(
            "INSERT INTO resource_specifications (id, name, unique_name, agent_unique_name) VALUES (?, ?, ?, ?)",
        )
        .bind(&ulid)
        .bind(new_resource_specification.name)
        .bind(unique_name)
        .bind(new_resource_specification.agent_unique_name)
        .execute(&context.pool)
        .await?;
        let inserted_resource_specification = sqlx::query_as::<_, ResourceSpecification>(
            "SELECT * FROM resource_specifications WHERE id = ?",
        )
        .bind(ulid)
        .fetch_one(&context.pool)
        .await?;
        Ok(inserted_resource_specification)
    }

    #[graphql(description = "Delete a resource specification")]
    async fn delete_resource_specification(
        context: &Context,
        unique_name: String,
    ) -> FieldResult<i32> {
        let result = sqlx::query("DELETE FROM resource_specifications WHERE unique_name = ?")
            .bind(unique_name)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    #[graphql(description = "Add new commitment")]
    async fn create_commitment(
        context: &Context,
        new_commitment: NewCommitment,
    ) -> FieldResult<Commitment> {
        let ulid = Ulid::new().to_string();
        // TODO put those in a transaction
        sqlx::query(
            "
            INSERT INTO commitments (id, description, process_id, action_id, assigned_agent_id, quantity, unit_id, resource_specification_id, due_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(&ulid)
        .bind(new_commitment.description)
        .bind(new_commitment.process_id)
        .bind(new_commitment.action_id)
        .bind(new_commitment.assigned_agent_id)
        .bind(new_commitment.quantity)
        .bind(new_commitment.unit_id)
        .bind(new_commitment.resource_specification_id)
        .bind(new_commitment.due_at)
        .execute(&context.pool)
        .await?;
        let mut inserted_commitment = sqlx::query("SELECT * FROM commitments WHERE id = ?")
            .bind(&ulid)
            .map(Commitment::from_row)
            .fetch_one(&context.pool)
            .await?;
        let action = sqlx::query_as::<_, Action>(
            "
           SELECT *
           FROM actions 
           WHERE actions.id = ?
           ",
        )
        .bind(&inserted_commitment.action_id)
        .fetch_one(&context.pool)
        .await?;
        inserted_commitment.action = Some(action);
        let unit = sqlx::query_as::<_, Unit>(
            "
            SELECT *
            FROM units 
            WHERE units.id = ?
            ",
        )
        .bind(&inserted_commitment.unit_id)
        .fetch_one(&context.pool)
        .await?;
        inserted_commitment.unit = Some(unit);
        let resource_specification = sqlx::query_as::<_, ResourceSpecification>(
            "
           SELECT *
           FROM resource_specifications
           WHERE resource_specifications.id = ?
           ",
        )
        .bind(&inserted_commitment.resource_specification_id)
        .fetch_one(&context.pool)
        .await?;
        inserted_commitment.resource_specification = Some(resource_specification);
        if let Some(assigned_agent_id) = &inserted_commitment.assigned_agent_id {
            let assigned_agent = sqlx::query_as::<_, Agent>(
                "
           SELECT *
           FROM agents
           WHERE agents.id = ?
           ",
            )
            .bind(assigned_agent_id)
            .fetch_one(&context.pool)
            .await?;
            inserted_commitment.assigned_agent = Some(assigned_agent);
        }
        Ok(inserted_commitment)
    }

    #[graphql(description = "Update a commitment")]
    async fn update_commitment(
        context: &Context,
        update_commitment: UpdateCommitment,
    ) -> FieldResult<i32> {
        let result = sqlx::query(
            "
            UPDATE commitments
            SET description = ?,
                unit_id = ?,
                action_id = ?,
                resource_specification_id = ?,
                quantity = ?,
                assigned_agent_id = ?,
                due_at = ?
            WHERE id = ?",
        )
        .bind(update_commitment.description)
        .bind(update_commitment.unit_id)
        .bind(update_commitment.action_id)
        .bind(update_commitment.resource_specification_id)
        .bind(update_commitment.quantity)
        .bind(update_commitment.assigned_agent_id)
        .bind(update_commitment.due_at)
        .bind(update_commitment.id)
        .execute(&context.pool)
        .await?;
        Ok(result.rows_affected() as i32)
    }

    #[graphql(description = "Delete a commitment")]
    async fn delete_commitment(context: &Context, id: String) -> FieldResult<i32> {
        let result = sqlx::query("DELETE FROM commitments WHERE id = ?")
            .bind(id)
            .execute(&context.pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}
