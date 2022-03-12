use super::{
    Action, Agent, AgentType, Commitment, Label, Plan, Process, ResourceSpecification, Unit,
};
use async_graphql::{Context, InputObject, Object, Result};
use futures::future::join_all;
use sqlx::sqlite::SqlitePool;
use ulid::Ulid;

fn unique_name(name: &str) -> String {
    name.to_string().to_lowercase().replace(" ", "_")
}

#[derive(InputObject, Debug)]
struct NewAgent {
    name: String,
    email: Option<String>,
    agent_type: AgentType,
}

#[derive(InputObject, Debug)]
struct NewPlan {
    title: String,
    agent_id: String,
    description: Option<String>,
}

#[derive(InputObject, Debug)]
struct UpdatePlan {
    id: String,
    title: String,
    description: Option<String>,
}

#[derive(InputObject, Debug)]
struct NewLabel {
    name: String,
    color: String,
}

#[derive(InputObject, Debug)]
struct NewProcess {
    title: String,
    description: Option<String>,
    plan_id: Option<String>,
    start_date: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
    agents: Option<Vec<String>>,
}

#[derive(InputObject, Debug)]
struct UpdateProcess {
    id: String,
    title: String,
    description: Option<String>,
    labels: Option<Vec<String>>,
    agents: Option<Vec<String>>,
}

#[derive(InputObject, Debug)]
struct NewResourceSpecification {
    name: String,
}

#[derive(InputObject, Debug)]
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

#[derive(InputObject, Debug)]
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

#[Object]
impl MutationRoot {
    async fn create_agent<'ctx>(
        &self,
        context: &Context<'ctx>,
        new_agent: NewAgent,
    ) -> Result<Agent> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(&new_agent.name);
        let inserted_agent = sqlx::query_as::<_, Agent>(
            "
            INSERT INTO agents (id, name, unique_name, email, agent_type)
            VALUES (?, ?, ?, ?, ?)
            RETURNING *
        ",
        )
        .bind(&ulid)
        .bind(new_agent.name)
        .bind(unique_name)
        .bind(new_agent.email)
        .bind(new_agent.agent_type)
        .fetch_one(pool)
        .await?;
        Ok(inserted_agent)
    }

    async fn delete_agent<'ctx>(
        &self,
        context: &Context<'ctx>,
        unique_name: String,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let result = sqlx::query!("DELETE FROM agents WHERE unique_name = ?", unique_name)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }
    async fn create_label<'ctx>(
        &self,
        context: &Context<'ctx>,
        new_label: NewLabel,
    ) -> Result<Label> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(&new_label.name);
        let inserted_label = sqlx::query_as!(
            Label,
            "INSERT INTO labels (id, name, unique_name, color)
            VALUES (?, ?, ?, ?)
            RETURNING *",
            ulid,
            new_label.name,
            unique_name,
            new_label.color
        )
        .fetch_one(pool)
        .await?;
        Ok(inserted_label)
    }

    async fn delete_label<'ctx>(&self, context: &Context<'ctx>, id: String) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let result = sqlx::query!("DELETE FROM labels WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn create_plan<'ctx>(&self, context: &Context<'ctx>, new_plan: NewPlan) -> Result<Plan> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        let inserted_plan = sqlx::query(
            "INSERT INTO plans (id, title)
            VALUES (?, ?)
            RETURNING *",
        )
        .bind(&ulid)
        .bind(new_plan.title)
        .map(Plan::from_row)
        .fetch_one(pool)
        .await?;
        sqlx::query("INSERT INTO plan_agents (plan_id, agent_id) VALUES (?, ?)")
            .bind(&ulid)
            .bind(new_plan.agent_id)
            .execute(pool)
            .await?;
        Ok(inserted_plan)
    }

    async fn update_plan<'ctx>(
        &self,
        context: &Context<'ctx>,
        update_plan: UpdatePlan,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let UpdatePlan {
            title,
            description,
            id,
            ..
        } = update_plan;
        let result = sqlx::query!(
            "UPDATE plans SET title = ?, description = ? WHERE id = ?",
            title,
            description,
            id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn create_process<'ctx>(
        &self,
        context: &Context<'ctx>,
        new_process: NewProcess,
    ) -> Result<Process> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        // TODO put those in a transaction
        let mut inserted_process = sqlx::query(
            "
            INSERT INTO processes (id, title, description, plan_id)
            VALUES (?, ?, ?, ?)
            RETURNING *
            ",
        )
        .bind(&ulid)
        .bind(new_process.title)
        .bind(new_process.description)
        .bind(new_process.plan_id)
        .map(Process::from_row)
        .fetch_one(pool)
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
                    .execute(pool)
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
                    .execute(pool)
            })
            .collect::<Vec<_>>();
        // TODO parallelize
        join_all(new_process_labels).await;
        join_all(new_process_agents).await;

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
        .fetch_all(pool)
        .await?;
        let agents = sqlx::query(
            "
            SELECT agents.id, name, unique_name, agent_type
            FROM agents
            INNER JOIN process_agents
            ON process_agents.agent_id = agents.id
            WHERE process_agents.process_id = ?
            ",
        )
        .bind(ulid)
        .map(Agent::from_row)
        .fetch_all(pool)
        .await?;
        inserted_process.labels = labels;
        inserted_process.agents = agents;
        Ok(inserted_process)
    }

    async fn update_process<'ctx>(
        &self,
        context: &Context<'ctx>,
        update_process: UpdateProcess,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        // TODO paralelize queries
        let id: String = update_process.id;
        sqlx::query!("DELETE FROM process_labels WHERE process_id = ?", id)
            .execute(pool)
            .await?;
        sqlx::query!("DELETE FROM process_agents WHERE process_id = ?", id)
            .execute(pool)
            .await?;
        let new_process_labels = update_process
            .labels
            .unwrap_or_else(Vec::new)
            .iter()
            .map(|label_id| {
                sqlx::query("INSERT INTO process_labels (process_id, label_id) VALUES (?, ?)")
                    .bind(&id)
                    .bind(label_id.clone())
                    .execute(pool)
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
                    .execute(pool)
            })
            .collect::<Vec<_>>();
        join_all(new_process_labels).await;
        join_all(new_process_agents).await;

        let result = sqlx::query("UPDATE processes SET title = ?, description = ? WHERE id = ?")
            .bind(update_process.title)
            .bind(update_process.description)
            .bind(&id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn delete_process<'ctx>(
        &self,
        context: &Context<'ctx>,
        process_id: String,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let mut transaction = pool.begin().await?;
        sqlx::query!(
            "DELETE FROM process_labels WHERE process_id = ?",
            process_id
        )
        .execute(&mut transaction)
        .await?;
        sqlx::query!(
            "DELETE FROM process_agents WHERE process_id = ?",
            process_id
        )
        .execute(&mut transaction)
        .await?;
        let result = sqlx::query!("DELETE FROM processes WHERE id = ?", process_id)
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.rows_affected() as i32)
    }

    async fn create_resource_specification<'ctx>(
        &self,
        context: &Context<'ctx>,
        new_resource_specification: NewResourceSpecification,
    ) -> Result<ResourceSpecification> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        let unique_name: String = unique_name(&new_resource_specification.name);
        let inserted_resource_specification = sqlx::query_as!(
            ResourceSpecification,
            "INSERT INTO resource_specifications (id, name, unique_name)
                VALUES (?, ?, ?)
                RETURNING *",
            ulid,
            new_resource_specification.name,
            unique_name
        )
        .fetch_one(pool)
        .await?;
        Ok(inserted_resource_specification)
    }

    async fn delete_resource_specification<'ctx>(
        &self,
        context: &Context<'ctx>,
        unique_name: String,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let result = sqlx::query!(
            "DELETE FROM resource_specifications WHERE unique_name = ?",
            unique_name
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn create_commitment<'ctx>(
        &self,
        context: &Context<'ctx>,
        new_commitment: NewCommitment,
    ) -> Result<Commitment> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let ulid = Ulid::new().to_string();
        // TODO put those in a transaction
        let mut inserted_commitment= sqlx::query(
            "
            INSERT INTO commitments (id, description, process_id, action_id, assigned_agent_id, quantity, unit_id, resource_specification_id, due_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
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
        .map(Commitment::from_row)
        .fetch_one(pool)
        .await?;
        let action = sqlx::query_as::<_, Action>(
            "
           SELECT *
           FROM actions 
           WHERE actions.id = ?
           ",
        )
        .bind(&inserted_commitment.action_id)
        .fetch_one(pool)
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
        .fetch_one(pool)
        .await?;
        inserted_commitment.unit = Some(unit);
        let resource_specification = sqlx::query_as!(
            ResourceSpecification,
            "
           SELECT *
           FROM resource_specifications
           WHERE resource_specifications.id = ?
           ",
            inserted_commitment.resource_specification_id
        )
        .fetch_one(pool)
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
            .fetch_one(pool)
            .await?;
            inserted_commitment.assigned_agent = Some(assigned_agent);
        }
        Ok(inserted_commitment)
    }

    async fn update_commitment<'ctx>(
        &self,
        context: &Context<'ctx>,
        update_commitment: UpdateCommitment,
    ) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let UpdateCommitment {
            id,
            description,
            action_id,
            quantity,
            unit_id,
            resource_specification_id,
            assigned_agent_id,
            due_at,
        } = update_commitment;
        let result = sqlx::query!(
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
            description,
            unit_id,
            action_id,
            resource_specification_id,
            quantity,
            assigned_agent_id,
            due_at,
            id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn delete_commitment<'ctx>(&self, context: &Context<'ctx>, id: String) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let result = sqlx::query!("DELETE FROM commitments WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }

    async fn delete_relationship<'ctx>(&self, context: &Context<'ctx>, id: String) -> Result<i32> {
        let pool = context
            .data::<SqlitePool>()
            .expect("failed to get connection pool");
        let result = sqlx::query!("DELETE FROM agent_relations WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() as i32)
    }
}
